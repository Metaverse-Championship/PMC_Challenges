# Polkadot Metaverse Championship - Challenges

The [Polkadot Metaverse Championship (PMC)](https://metaversechampionship.gg/) is an open web3/cryptocurrency contest consisting of a qualification phase and a main event.

# Qualification

To qualify, you must solve at least 3 challenges from the 5 provided in this repository.

### The process

Each challenge has its own subfolder in the "Prequalification_Challenges" folder, wherein it is described.

The format of a solution is an ASCII string of the form __PMC{...}__, like PMC{#\This_Could_be_Flagz/#}. A correct solution is also called a flag. What you are looking for in each challenge is to generate the string between PMC{......}.

[Submit your solutions in this web form](https://www.cognitoforms.com/UnconditionalPeace/PolkadotMetaverseChampionshipPrequalification) to ask for an invitation.

(If you've played Capture The Flag (CTF) games before, this process should be familiar to you.)

## Offline flag verification

You can verify the correctness of your solutions before submitting the form. The flags have the following respective Keccak-256 hash values:

```
kusamaverse - 0x896c90f019d0aaa7977ce81c7d7299b1b43d302295f2d567509ab7e3060a797f
wss - 0x2f2d4bb11521956c486925241ffcca0cbf7b79bbd9be8eafaeb4fab95713b12d
solidity - 0x81b1ff6939fedd672ddac358a41abb7192ae1f0660a3b1cbf92ef2c82119612b
ink - 0x82703a464305aad655e2eb617f31e6e57b7e959bf8528f1d3b5968cc02ed60ac
rust - 0x42ce6fca873fe4dc4ce4d9accdb53e02fdb497ffa6b30f421cea36c81d8ea289
```

To calculate the Keccak-256 hash of a string, you can, for example, install `keccak256-cli` using `cargo`, and then use the `keccak256` command like so:
```
echo 'PMC{This_Could_be_a_Flag.!}'|keccak256
0x6104a5489b671452684fee77579df24e6f2441d1f8a832c7366cf2cfc5194935
```

## List of prequalification challenges

__WSS challenge__: you need to play with a websocket connection and get the flag from the service. It is called Fibonacci.

__Solidity challenge__: a security audit might reveal what the developers think is private might actually not be.

__Rust challenge__: work with RSA cryptography and math. The correct math will return you the flag.

__ink! challenge__: you need to get the flag from an already running Substrate node or find "another" way. Make sure to web2 scan everything.

__Kusamaverse challenge__: find the flag in Kusamaverse - this one is very easy, but important for the main event.

We also have an __optional 3D design challenge__ for the Hall of Fame, near the five listed above. If you submit it using the prequalification form, we'll add the design to the Hall of Fame, where all the successfully prequalified players get listed.

### Design for fame

By the way, you can compound your solutions with an optional 3D design of your own, in the web form. We'll add each such design to the Hall of Fame, where all the successfully qualified players get listed.

## Challenges for the main event

The details of the main challenges will be shared on 2022 December 6th 10:00 CET. The teams formed before will need to solve at least one from each track, in 24 hours. You can already prepare libraries and some code before the hackathon, but the final ideation and project needs to come out during the main event.

## Main event

The main event consists of a 3-day hackathon where projects can be conceptualized and implemented, and also of more challenge-solving in parallel.

At first, teams should be formed by qualified players.

The main event's challenges will be disclosed on 2022-12-06 at 10:00:00 CET. The teams will need to solve at least one from each challenge track, in 24 hours.

Note: You can already prepare libraries and some code before the hackathon, but the final project needs to be conceived during the main event.

### Main event challenges

An outline:

#### Track 1 - Collaboration

Challenge 1 - Momentum's Kusamaverse related

Challenge 2 - Unit Network related

#### Track 2 - Privacy and Security

Challenge 1 - CCTF project related

Challenge 2 - Research related

#### Track 3 - GameFi and NFT

Challenge 1 - RMRK project related

Challenge 2 - KILT related

## Support and feedback for the prequalification

We don't help with solving the challenges, but if something is unclear or you think you've found a bug, we are always open to hear from you.

## Contact

You can contact us anytime though our email address contact@metaversechampionship.gg or you can talk to __six__ through Matrix (@hexff:matrix.org).

You can find all our communication channels, media content and PR postings through [our linkfree](https://linkfree.metaversechampionship.gg/).
