#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

extern crate rendering;
extern crate tessellation;

pub mod benchmarks;
pub mod driver;
// TODO: Option builder for analysis.

#[cfg(test)]
#[test]
fn test() {
    let builder = driver::RunOptions::builder();
    let builder = builder.add(|| benchmarks::tessellation::profile::test(5));
    let builder = builder.add(|| benchmarks::tessellation::profile::test(6));

    let driver = driver::Driver::from(builder.build());
    driver.run();
}
