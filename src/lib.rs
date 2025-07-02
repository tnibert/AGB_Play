#![no_std]

// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

pub mod sprite;
pub mod player;
pub mod gameobject;
pub mod movingstone;

pub const BALL_SIZE: i32 = 16;
