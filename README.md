# fingers: a finger clone for Slack

[![Build Status](https://travis-ci.org/julienXX/fingers.svg?branch=master)](https://travis-ci.org/julienXX/fingers)
[![Build status](https://ci.appveyor.com/api/projects/status/r9b13pq29g7n4ux0?svg=true)](https://ci.appveyor.com/project/julienXX/fingers)

## Installation
Clone the repository then:
```
λ cargo build --release
λ cp target/release/fingers /usr/local/bin
```

## Usage
```
λ export SLACK_TOKEN=...
λ fingers julien

 Login      Name               Title   Email                 Skype   Phone   Timezone           Presence
 julien     julien blanchard           julien@sideburns.eu                   Europe/Amsterdam   active


```

## Note
It doesn't work for bots yet.
