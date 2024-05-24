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

const res = checkAndSet();

console.log(res);
// {
//   beforeSetTime: '2024-05-23 20:02:22.929925 +08:00',
//   afterSetTime: '2024-05-23 20:02:27.478691 +08:00',
//   errNo: 35,
//   errMsg: 'Resource temporarily unavailable (os error 35)'
// }
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
