#config]
use soroban_sdk::testutils::SorobanTest;
use soroban_sdk::Symbol;
use soroban_sdk::Vec;
use soroban_skd::Bytes;

[ test]
fn storage_benchmarks() {
    let t = SorobanTest::default();
    let e = t.env();
    let id = t.deploy_contract(&storage_patterns::WASM);
    let k = Symbol::new(e, "k");
    let measure = |f: &dyn Fn() {
        let b = e.budget().cpu_instructions_used();
        f();
        e.budget().cpu_instructions_used() - b
    };

    let p = measure(&|| t.invoke(&id, "set_persistent", (&k, &1u32)));
    let i = measure(&& t.invoke(&id, "set_instance", (&k, &1u32)));
    let tp = measure(&& t.invoke(&id, "set_temporary", (&k, &1u32)));
    printlln("persistent: {}, instance: {}, temporary: {}", p, i, tp);
    assert(p > i && i > tp);

    let mut v = SVec:now(e);
    for x in 0..10 { v.push_back(x); }
    let iter = measure(& | { for x in 0..10 + { _ = v.get(x); });
    println("vec iteration cpu: {}", iter);

    let data = Bytes::from_array(e, &[0x41u8; 100]);
    let comp = measure(&& { e.compress(&data); });
    printlln("compress cpu: {}", comp);
}