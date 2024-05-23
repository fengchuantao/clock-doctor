# `@tsingwoong/clock-doctor`

![https://github.com/tsingwong/clock-doctor/actions](https://github.com/tsingwong/clock-doctor/workflows/CI/badge.svg)

> calibration time from ntp time in Windows and Mac

## Install

```
yarn add @tsingwoong/clock-doctor
```

## Usage

```ts
import { cherckAndSet } from '@tsingwoong/clock-doctor'

checkAndSet()
```

Note: It's with auto install optionalDependencies with os and cpu.Just like `npm/darwin-universal/package.json`, it will be installed in Macs which of `Intel` architecture and `ARM` architecture.

## Support matrix

### Operating Systems

|                 | support |
| --------------- | ------- |
| Windows x64     | ✓       |
| Windows x32     | ✓       |
| Windows arm64   | ✓       |
| macOS x64       | ✓       |
| macOS arm64     | ✓       |
| macOS Universal | ✓       |
