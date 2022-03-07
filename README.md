# Drew's Foundation bindings for Rust

This library binds (some subset of) Apple Foundation to Rust.  It may be compared to [objc-foundation](https://crates.io/crates/objc-foundation/0.1.1/dependencies)
and [cocoa-foundation](https://crates.io/crates/cocoa-foundation).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features
of foundationr:

* Zero-cost abstractions.  Calling this library should perform identically to calling Foundation from Swift/ObjC applications.
    * Most of the magic happens in [objr](https://github.com/drewcrawford/objr) or [blocksr](https://github.com/drewcrawford/blocksr)
      which provide cutting-edge high-performance primitives which are used here extensively.
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with Foundation APIs and are not documented separately.
* Modular.  Foundation is a big library; individual features are gated with feature flags
* Free for noncommercial or "small commercial" use.

# Table of features

The scope of this project is to implement "all of foundation", however, in practice, I mostly have time
to implement APIs I actually use in other projects.

The following cargo features are currently supported, and provide at least some APIs on each type:

## Core types

* `nsstring`
* `nsdata`
* `nsdictionary`
* `nsvalue`
* `nsrange`
* `nsdate`

## Runtime

* `nscopying`

## System
* `nsthread`
* `nsnotification`
* `nsrunloop`


## HTTP and URLs
* `nsurl`
* `nsurlresponse`
* `nsurlrequest`
* `nsurlsession`

## Other
* `all` - enables all other features
