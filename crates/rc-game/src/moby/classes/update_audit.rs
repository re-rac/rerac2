//! The class-update audit: every class whose update is one original function must run one port update.
//!
//! For every level 00..26 and every class the census lists (`work/census/classes.tsv`: the class's v2.00 update per
//! level), the original update is identified across levels through the decomp's catalogues
//! (`work/ref/rac2-decomp/config/us2/levels/functions.tsv`, `config/us2/boot/functions.tsv`; the decomp numbers
//! `G/LEVEL21.WAD`, our level 21, as its level 30).
//!
//! A function's identity is its code: the catalogue's fingerprint and size (its words with the addresses masked). Where
//! several catalogue rows share that code (the k-th copy in each level is one row), the catalogue's names are not enough:
//! the one-call wrappers (`jal X; return`, X masked) of different targets share names across levels. Those are told
//! apart by the identities of the functions they call (`jal`, read from `work/ghidra-import/levelNN.elf`).
//! Each function's classes are then compared by what [`super::update_for`] returns for them.
//!
//! The report goes to `work/census/update_audit.txt`:
//! (a) functions whose classes run different port updates, or some none (the test fails on these, except the
//!     known per-level cases listed in `LEVEL_DEPENDENT`);
//! (b) functions none of whose classes is ported, by placed count;
//! (c) classes whose update is a different original function on different levels (the port has one update per class).
//!
//! Run: `python3 work/testgate/tg.py request <agent> test rc-game update_audit` (the `work/` files are local; the
//! test passes with a note when they are missing).

use std::collections::{BTreeMap, BTreeSet, HashMap};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../work");

/// Classes whose update is another original function on some levels than on others (section (c)), so that the
/// port's one update per class cannot match every level.
const LEVEL_DEPENDENT: &[i16] = &[];

struct Row {
    level: String,
    class: i16,
    placed: u32,
    created: u32,
}

fn rows(path: &str) -> Option<Vec<Vec<String>>> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(
        text.lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| l.split('\t').map(str::to_string).collect())
            .collect(),
    )
}

/// An ELF's loaded segments (vaddr, bytes).
struct Elf(Vec<(u32, Vec<u8>)>);

impl Elf {
    fn load(path: &str) -> Option<Elf> {
        let d = std::fs::read(path).ok()?;
        let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
        let u16_at = |o: usize| u16::from_le_bytes(d[o..o + 2].try_into().unwrap()) as usize;
        let (phoff, phsz, phn) = (u32_at(0x1c) as usize, u16_at(0x2a), u16_at(0x2c));
        let mut segs = Vec::new();
        for i in 0..phn {
            let p = phoff + i * phsz;
            let (ty, off, va, fs) = (u32_at(p), u32_at(p + 4) as usize, u32_at(p + 8), u32_at(p + 16) as usize);
            if ty == 1 && off + fs <= d.len() {
                segs.push((va, d[off..off + fs].to_vec()));
            }
        }
        Some(Elf(segs))
    }

    fn word(&self, a: u32) -> Option<u32> {
        self.0.iter().find_map(|(va, b)| {
            let o = a.checked_sub(*va)? as usize;
            (o + 4 <= b.len()).then(|| u32::from_le_bytes(b[o..o + 4].try_into().unwrap()))
        })
    }
}

struct Catalogue {
    /// (decomp level, v2 address) -> (name, size, fingerprint).
    places: HashMap<(u8, u32), (String, u32, String)>,
    /// v2 boot address -> name.
    boot: HashMap<u32, String>,
}

fn catalogue() -> Option<Catalogue> {
    let mut places = HashMap::new();
    for r in rows(&format!("{ROOT}/ref/rac2-decomp/config/us2/levels/functions.tsv"))? {
        let size: u32 = r.get(2)?.parse().ok()?;
        for p in r.get(5).map(String::as_str).unwrap_or("").split(',').filter(|p| !p.is_empty()) {
            let (l, a) = p.split_once(':')?;
            places.insert((l.parse().ok()?, u32::from_str_radix(a, 16).ok()?), (r[0].clone(), size, r[3].clone()));
        }
    }
    let mut boot = HashMap::new();
    for r in rows(&format!("{ROOT}/ref/rac2-decomp/config/us2/boot/functions.tsv"))? {
        if let Some(a) = r.get(1).and_then(|a| u32::from_str_radix(a, 16).ok()) {
            boot.insert(a, r[0].clone());
        }
    }
    Some(Catalogue { places, boot })
}

/// Our level number -> the decomp's.
fn decomp_level(level: u8) -> u8 {
    if level == 21 { 30 } else { level }
}

/// A function in a level's program: (our level, v2 address).
type Node = (u8, u32);

struct Identity<'a> {
    cat: &'a Catalogue,
    elfs: HashMap<u8, Elf>,
}

impl Identity<'_> {
    fn base(&self, (level, a): Node) -> String {
        if let Some((_, size, fp)) = self.cat.places.get(&(decomp_level(level), a)) {
            format!("{fp}/{size}")
        } else if let Some(n) = self.cat.boot.get(&a) {
            format!("boot:{n}")
        } else {
            format!("{level:02}:{a:08X}")
        }
    }

    fn name(&self, (level, a): Node) -> String {
        match self.cat.places.get(&(decomp_level(level), a)) {
            Some((n, _, _)) => n.clone(),
            None => self.cat.boot.get(&a).cloned().unwrap_or_else(|| format!("{level:02}:{a:08X}")),
        }
    }

    /// The functions a level function calls (`jal`), in order; boot functions are not refined (one program).
    fn callees(&self, (level, a): Node) -> Vec<Node> {
        let Some((_, size, _)) = self.cat.places.get(&(decomp_level(level), a)) else { return Vec::new() };
        let Some(elf) = self.elfs.get(&level) else { return Vec::new() };
        (0..size / 4)
            .filter_map(|i| {
                let pc = a + i * 4;
                let w = elf.word(pc)?;
                (w >> 26 == 3).then(|| (level, ((pc + 4) & 0xf000_0000) | ((w & 0x03ff_ffff) << 2)))
            })
            .collect()
    }
}

/// A function's identity: its code (the catalogue's fingerprint and size), and where that code is not one catalogue
/// function (several rows share it: the k-th copy rule), the identities of the functions it calls too.
struct Identities<'a> {
    id: &'a Identity<'a>,
    /// (fingerprint, size) -> the catalogue rows with that code.
    copies: HashMap<(String, u32), u32>,
    intern: HashMap<(String, Vec<u32>), u32>,
    memo: HashMap<Node, u32>,
}

impl Identities<'_> {
    fn key(&mut self, n: Node, depth: u32) -> u32 {
        if let Some(&k) = self.memo.get(&n) {
            return k;
        }
        let base = self.id.base(n);
        let ambiguous = self.id.cat.places.get(&(decomp_level(n.0), n.1)).is_some_and(|(_, size, fp)| self.copies[&(fp.clone(), *size)] > 1);
        let callees = if ambiguous && depth < 8 {
            self.id.callees(n).into_iter().map(|m| self.key(m, depth + 1)).collect()
        } else {
            Vec::new()
        };
        let next = self.intern.len() as u32;
        let k = *self.intern.entry((base, callees)).or_insert(next);
        self.memo.insert(n, k);
        k
    }
}

#[test]
fn update_audit() {
    let (Some(census), Some(cat)) = (rows(&format!("{ROOT}/census/classes.tsv")), catalogue()) else {
        eprintln!("update_audit: work/census or work/ref/rac2-decomp missing; skipped");
        return;
    };
    let elfs: HashMap<u8, Elf> =
        (0..=26u8).filter_map(|l| Some((l, Elf::load(&format!("{ROOT}/ghidra-import/level{l:02}.elf"))?))).collect();
    if elfs.len() < 27 {
        eprintln!("update_audit: work/ghidra-import/levelNN.elf missing; skipped");
        return;
    }
    let id = Identity { cat: &cat, elfs };

    let parsed: Vec<(Node, Row)> = census
        .iter()
        .skip(1)
        .map(|r| {
            let level: u8 = r[0].parse().unwrap();
            let node = (level, u32::from_str_radix(&r[4], 16).unwrap());
            let row = Row { level: r[0].clone(), class: r[1].parse().unwrap(), placed: r[2].parse().unwrap_or(0), created: r[3].parse().unwrap_or(0) };
            (node, row)
        })
        .collect();
    let mut copies: HashMap<(String, u32), u32> = HashMap::new();
    for r in rows(&format!("{ROOT}/ref/rac2-decomp/config/us2/levels/functions.tsv")).unwrap() {
        *copies.entry((r[3].clone(), r[2].parse().unwrap())).or_default() += 1;
    }
    let mut ids = Identities { id: &id, copies, intern: HashMap::new(), memo: HashMap::new() };
    let ident: HashMap<Node, u32> = parsed.iter().map(|(n, _)| (*n, ids.key(*n, 0))).collect();

    // function id -> its (level, class) rows, and the decomp names of its places.
    let mut groups: BTreeMap<u32, (Vec<Row>, BTreeSet<String>)> = BTreeMap::new();
    let mut by_class: BTreeMap<i16, BTreeSet<u32>> = BTreeMap::new();
    for (node, row) in parsed {
        let k = ident[&node];
        by_class.entry(row.class).or_default().insert(k);
        let g = groups.entry(k).or_default();
        g.1.insert(id.name(node));
        g.0.push(row);
    }
    let fname = |k: u32| {
        let names = &groups[&k].1;
        let mut s: Vec<String> = names.iter().take(3).cloned().collect();
        if names.len() > 3 {
            s.push(format!("+{}", names.len() - 3));
        }
        s.join("|")
    };

    let port = |c: i16| super::update_for(c).map(|f| f as usize);
    // A port update's label: the classes (anywhere in the census) that run it.
    let mut label: HashMap<usize, BTreeSet<i16>> = HashMap::new();
    for &c in by_class.keys() {
        if let Some(p) = port(c) {
            label.entry(p).or_default().insert(c);
        }
    }
    let pname = |p: Option<usize>| match p {
        None => "None".to_string(),
        Some(p) => {
            let cs: Vec<String> = label[&p].iter().take(6).map(|c| format!("{c:#06x}")).collect();
            format!("fn[{}{}]", cs.join(","), if label[&p].len() > 6 { ",…" } else { "" })
        }
    };

    let mut a = Vec::new();
    let mut a_fail = 0;
    let mut b = Vec::new();
    let (mut whole, mut ported_groups) = (0, 0);
    for (&k, (members, _)) in &groups {
        let classes: BTreeSet<i16> = members.iter().map(|m| m.class).collect();
        let ports: BTreeMap<i16, Option<usize>> = classes.iter().map(|&c| (c, port(c))).collect();
        let distinct: BTreeSet<Option<usize>> = ports.values().copied().collect();
        let placed: u32 = members.iter().map(|m| m.placed).sum();
        let created: u32 = members.iter().map(|m| m.created).sum();
        let levels: BTreeSet<&str> = members.iter().map(|m| m.level.as_str()).collect();
        let levels: Vec<&str> = levels.into_iter().collect();
        if distinct.len() > 1 {
            let mut line = format!("{}\tplaced {placed}\tcreated {created}\tlevels {}", fname(k), levels.join(","));
            for (c, p) in &ports {
                let cp: u32 = members.iter().filter(|m| m.class == *c).map(|m| m.placed).sum();
                let cl: BTreeSet<&str> = members.iter().filter(|m| m.class == *c).map(|m| m.level.as_str()).collect();
                line += &format!("\n    {c:#06x} ({c}) placed {cp} levels {} -> {}", cl.into_iter().collect::<Vec<_>>().join(","), pname(*p));
            }
            if !classes.iter().any(|c| LEVEL_DEPENDENT.contains(c)) {
                a_fail += 1;
            } else {
                line += "\n    (level-dependent class: allowed)";
            }
            a.push(line);
        } else if distinct.contains(&None) {
            let cs: Vec<String> = classes.iter().map(|c| format!("{c:#06x}")).collect();
            b.push((placed, created, format!("{}\tplaced {placed}\tcreated {created}\tclasses {}\tlevels {}", fname(k), cs.join(","), levels.join(","))));
        } else {
            ported_groups += 1;
            if classes.len() > 1 {
                whole += 1;
            }
        }
    }
    b.sort_by(|x, y| y.0.cmp(&x.0).then(y.1.cmp(&x.1)));
    let mut c = Vec::new();
    for (class, keys) in &by_class {
        if keys.len() > 1 {
            let per: Vec<String> = keys
                .iter()
                .map(|k| {
                    let lv: BTreeSet<&str> = groups[k].0.iter().filter(|m| m.class == *class).map(|m| m.level.as_str()).collect();
                    format!("{} [{}]", fname(*k), lv.into_iter().collect::<Vec<_>>().join(","))
                })
                .collect();
            c.push(format!("{class:#06x} ({class}) -> {}\n    {}", pname(port(*class)), per.join("\n    ")));
        }
    }

    let mut out = format!(
        "# update audit: {} original update functions, {} fully ported ({} of them with several classes), {} mismatched (a), {} unported (b)\n\n",
        groups.len(),
        ported_groups,
        whole,
        a.len(),
        b.len()
    );
    out += "## (a) one original function, different port updates\n";
    for l in &a {
        out += l;
        out.push('\n');
    }
    out += "\n## (b) no class ported (by placed, created)\n";
    for (_, _, l) in &b {
        out += l;
        out.push('\n');
    }
    out += "\n## (c) classes whose original update differs by level (one port update per class)\n";
    for l in &c {
        out += l;
        out.push('\n');
    }
    let _ = std::fs::write(format!("{ROOT}/census/update_audit.txt"), &out);
    assert!(a_fail == 0, "{a_fail} original functions run different port updates (work/census/update_audit.txt):\n{}", a.join("\n"));
}
