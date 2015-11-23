# fingers: a finger clone for Slack

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
