cargo clippy --tests


cd ft-sdk || exit
cargo clippy --features=auth-provider,field-extractors --tests
cargo clippy --features=auth-provider,field-extractors,postgres-default --no-default-features --tests


