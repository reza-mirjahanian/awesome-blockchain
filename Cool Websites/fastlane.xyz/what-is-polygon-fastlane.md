Overview
========

FastLane Protocol Overview

The FastLane protocol (AKA 'Polygon FastLane' or 'PFL') is designed to enhance the Polygon blockchain by reducing transaction spam.

Nodes based on GETH, such as those run on Polygon PoS and BNB chain, create an inherent correlation between transaction quantity and trading success that incentivizes searchers to spam the network. The FastLane protocol works by overriding this mechanism with a highly efficient and transparent auction system.

Reducing spam can only be accomplished with the help of Polygon's validators. Onboarding for validators is as simple as [applying a one-line patch to their sentry nodes](https://fastlane-labs.gitbook.io/polygon-fastlane/getting-started-as-a-validator/patching-your-sentry-nodes-with-the-fastlane-patch) and then [adding the nearest FastLane node as an additional sentry](https://fastlane-labs.gitbook.io/polygon-fastlane/getting-started-as-a-validator/connecting-to-a-fastlane-sentry-node). Participating validators are rewarded for safeguarding the network by receiving the proceeds of any auctions occurring during their blocks.



Design Principles
=================

The FastLane protocol was built around seven core design principles:

-   User transactions are always as fast as (or faster than) Polygon's "default" value.

-   The removal of the correlation between transaction quantity and trading success.

-   No added protections for those who would attack Polygon's users.

-   No custom code for validator nodes and only a one-line patch for sentry nodes.

-   Validator liveness and "checkpointing" have neither infrastructure nor codebase dependencies on FastLane Labs.

-   No additional security vulnerabilities or attack surfaces for validators.

-   Extremely fast and frictionless validator onboarding