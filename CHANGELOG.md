# Changelog

All notable changes to this project will be documented in this file.

## [0.2.2] - 2025-11-20

### Features

- Simpler approach to `router` ([da785d3](https://github.com/azzamsa/tun/commit/da785d3d07fb2f9bd269ca011d3dc6eb385404b0))

  I prefer this approach.
  - No more multiple `Arc::clone()`

  I take it as Utoipa limitation(?)

- Add github `zen` ([02ab743](https://github.com/azzamsa/tun/commit/02ab743d3db11c250e4984efcc0fd868fc3b9436))
- Add `driver` boilerplate ([1105d32](https://github.com/azzamsa/tun/commit/1105d32a220f92a791365181e7f714b5f609a8a3))

  Demonstrate how to interact with external services.

### Bug fixes

- Avoid duplicate `with_state()` ([ca891eb](https://github.com/azzamsa/tun/commit/ca891eb2fed9c9569c643384399931842a00f4e1))

## [0.2.0] - 2025-11-19

### Features

- More `meta` info ([0d08e62](https://github.com/azzamsa/tun/commit/0d08e628e66d26cdb3a6ed966d5e27c2e8b6f858))
- Create `user` ([28c8689](https://github.com/azzamsa/tun/commit/28c8689790fb0ba37616ba739c497e3541215efe))
- Update `user` ([fa58d5b](https://github.com/azzamsa/tun/commit/fa58d5bd51c349f7662020303d3031f071bba554))
- Delete `user` ([7b82de4](https://github.com/azzamsa/tun/commit/7b82de47b2df8e9c2ebaa841d04a72e64ec340d4))
- Get `user` ([08bd728](https://github.com/azzamsa/tun/commit/08bd728ec402298d4e475ba59eed9f08d791401a))
- Get users ([298ec9e](https://github.com/azzamsa/tun/commit/298ec9e3f67315f11963e0b3027b1dce78a19876))
- Initial migration scaffolding ([be726a3](https://github.com/azzamsa/tun/commit/be726a3fb7a965fbf9bb82a8039300d960b99eda))
- Initial database scaffolding ([63f372e](https://github.com/azzamsa/tun/commit/63f372e1b2756d53b12fbb6d7dec8fd39e588fb6))
- Simpler `config` ([2b285c4](https://github.com/azzamsa/tun/commit/2b285c4e0f5ae46c632eb8a6d04cdac47c7fe3fa))

### Bug fixes

- Don't set local timezone in the log ([c777665](https://github.com/azzamsa/tun/commit/c7776654f0c300511e059024a527f54cc20a9d90))
- Status code is more than enough for `/health` ([c9750bb](https://github.com/azzamsa/tun/commit/c9750bbca25877b35132f0998800224f81adde70))

## [0.1.0] - 2023-01-12

### Features

- Add health resource ([bce456e](https://github.com/azzamsa/tun/commit/bce456e9ea32c9bde5a852ce2b863b29cf67bc06))
- Add swagger documentation ([6626785](https://github.com/azzamsa/tun/commit/6626785920f41e384208efb258860bf0f5d5f402))

### Bug fixes

- Utoipa can't find `model::<StructName>` ([322fa2b](https://github.com/azzamsa/tun/commit/322fa2b5a9b17c71f0ff32f01ae8429345a5e6ee))
