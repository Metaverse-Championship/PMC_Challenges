# Unit Challenge for Polkadot Metaverse Championship

## Unit Network - Collaboration Track Challenges

Participants may choose any one of the three challenges below to complete in a 24hr period. 

Each challenge relates to a combination and/or extension of existing Substate palettes.

An emphasis is to provide the freedom and incentive for each participant to produce something useful related to any of the three options below. As such these are generic overviews to explain the desired application of each palette. 

Participants will be judged on the novelty, simplicity and security of their solutions. Please chose only one from the challenges below. 

## 1. Badges Module

### Challenge 

Create the simplest and most secure Badges Module in 24hrs. The three essential components are:

- Non-transferrable once received from the creator
-¢ Can be set to expire within a specified timeframe
-¢ Can be batch transferred to selected account / wallets

Note: Any additional parameters that enhance the usability and feature set of the Badges Module will also be considered favourably by the judges.


### Overview 

The Badges Module provides functionality for asset management of â€˜conditionalâ€™ fungible asset classes akin to Proof of Attendance Protocol tokens (POAPs) and Soulbound Tokens (SBTs) including:

-¢ Asset Issuance
-¢ Asset Transfer (limited to a single transfer, can be batch transferred)
-¢ Asset Expiry (set by creator but not required)
-¢ Asset Destruction (if received but unwanted)


### Terminology

- Asset Issuance: The creation of a new asset instance
- Asset Transfer: The action of transferring an asset instance from one account to another
- Asset Burning: The destruction of an asset instance
- Fungible Asset: An asset whose units are interchangeable 
- Conditional Fungible Asset: An asset whose units are interchangeable but have conditions set by the creator to specify functionality


### Goals

The badges system in Substrate is designed to make the following possible:

- Issue a conditional asset to itâ€™s creators account
- Move asset to another account (one time) non-transferable after being received 
- Optionality to set an expiry date of the asset after which it is burned and removed from the total supply. 
- Remove an accounts balance of an asset when requested by the asset creator and update the assets total supply.


### Usage

The following example shows how to use the Badges Module in your runtime by exposing public functions to:

-  Issue a new conditional fungible asset set for a badge distribution event (airdrop)
-  Query the conditional fungible asset holding balance of an account
-  Query the total supply of a fungible asset that has been issued


### Use-cases

Similar to SBTs and POAPs, the Badges Module may be used to:

-  Issue non-transferrable certificates (badges) to graduates of a course or program (Additionally with an option for the badges to expire, requiring the graduates to do a course refresher or to earn a new valid / current badge.)

- Issue non-transferrable proof-of-attendance at an event 

- Issue a non-transferrable proof-of-participation / position in a community, team or initiative

- Issue a non-transferrable ticket to access to an event 


## 2. Subscription Module

###Challenge

Create the simplest and most secure Subscription Module in 24hrs. The three essential components are:

-  Allow authorisation of a recurring fungible asset debit, signed in a single transaction based on a time period set by subscription creator
-  Subscription can be cancelled on either side, by the user wallet or subscription creator
-  Query the success of each recurring asset transfer with a clear output value 

Note: Any additional parameters that enhance the usability and feature set of the Subscription module will also be considered favourably by the judges.

### Overview

The subscription module provides functionality to create an automated recurring asset debit from authorised wallets for a set period of time, or until cancelled by the wallet owner or subscription creator including:

-  Subscription Creation
-  Subscription Details
-  Subscription Plan
-  Subscription Start Date
-  Subscription Expiry Date
-  Subscription Amount
-  Subscription Period 
-  Subscription Period Description
-  Subscription Unique
-  Subscription Cancellation

### Terminology

-  Subscription Creation: The creation of a new subscription instance
-  Subscription Details: Holds the details of the subscription instance
-  Subscription Plan: Type of subscription plan
-  Subscription Start Date: (Optional) Date that the user will first be billed. (will default to the day the subscription is created if no startDate is specified)
-  Subscription Expiry Date: (Optional) Date the last payment is processed
-  Subscription Amount: Value in defined asset that the user pays at specified frequency
-  Subscription Period: How often the user will be billed
-  Scheduled Asset Transfer: The Automated Direct debit of assets from an authorised user wallet to a wallet defined by the subscription creator
-  Subscription Unique: This value allows a single account to have more than one subscription
-  Subscription Cancellation: Can be cancelled by either user wallet or subscription creator

### Goals

The Subscription module in Substrate is designed to make the following possible:

-  Create a recurring debit contract between user wallets and a subscription creator
-  Define subscription parameters 
-  Query the success of each recurring asset transfer with a clear output value 


### Use-cases

-  Create an automatic, token powered paywall to manage access to content or communities

-  Create a community / patron subscription model
