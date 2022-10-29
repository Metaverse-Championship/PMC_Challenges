# Polkadot Metaverse Championship - Challenges

## How to qualify and get a pass to the physical event?

Solve at least three challenges from the four provided in this repository and submit them in the form linked below.

You can find all the four prequalification challenges in the "Prequalification_Challenges" folder. If you played Capture The Flag /CTF/ games before, this process is very similar to you already.

The flag format: __PMC{This_Could_be_a_Flag.!}__

What you are looking for in each challenge is to generate the data between PMC{......}.

After you solve at least three challenges, you can submit the form and request your invitation:
__[Prequalify me!](https://www.cognitoforms.com/UnconditionalPeace/PolkadotMetaverseChampionshipPrequalification)__

## How to verify a flag?

You can verify your flag is correct without submitting the form. Install keccak256 with cargo and compare your findings. Example command:

```
echo 'PMC{This_Could_be_a_Flag.!}'|keccak256
```

The output of the example command should be: 0x6104a5489b671452684fee77579df24e6f2441d1f8a832c7366cf2cfc5194935

The flags have the following keccak256 values:

```
kusamaverse - 0x896c90f019d0aaa7977ce81c7d7299b1b43d302295f2d567509ab7e3060a797f
ink - 0x82703a464305aad655e2eb617f31e6e57b7e959bf8528f1d3b5968cc02ed60ac
wss - 0x2f2d4bb11521956c486925241ffcca0cbf7b79bbd9be8eafaeb4fab95713b12d
rust - 0x42ce6fca873fe4dc4ce4d9accdb53e02fdb497ffa6b30f421cea36c81d8ea289
python - 0xf00b792cd63a035c76abe2fca4afe8d8b7eb8527b9b547b672545c71ee4ca77f

```

## Prequalification challenges

__WSS challenge__: you need to play with a websocket connection and get the flag from the service.

__Rust challenge:__ you need to work with cryptography and math. The correct math will return you the flag.

__ink! challenge__: you need to get the flag from an already running Substrate node.

__Kusamaverse challenge__: find the flag in Kusamaverse - this one is very easy, but important for the main event.

We also have an __optional 3D design challenge__ for the Hall of Fame, near the four listed above. If you submit it using the form, we'll add the design to the Hall of Fame, where all the successfully prequalified players get listed.

## Support and feedback for the prequalification

We don't help with the solution. If something is unclear or you need help or think you found a bug, we are always open to hear from you.
You can contact us anytime though our email address contact@metaversechampionship.gg or you can also find six through Matrix (:hexff@matrix.org).

## Challenges for the main event

The details of these challenges will be shared on 2022 December 6th 10:00 CET. The teams will need to solve at least one from each track. You can already prepare libraries and some code befor the hackathon, but the final ideation and project needs to come out during the main event.

### Track 1 - Collaboration

Challenge 1 - by Momentum

Challenge 2 - Creator to be disclosed.

### Track 2 - Privacy and Security

Challenge 1 - by CCTF

Challenge 2 - Creator to be disclosed.

### Track 3 - GameFi and NFT

Challenge 1 - by RMRK

Challenge 2 - Creator to be disclosed.
