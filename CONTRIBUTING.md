# Contributing to Epsonia

## Introduction

*What is Epsonia?*

Epsonia is a scoring engine based on the cyberpatriot competition which
searches an environment for vulnerabilities and scores based on the status of the vulnerabilities *(fixed, not fixed, or penalized)*

*What does Epsonia need?*

Epsonia needs various checks in order to expand options for people making
practice images or competition images.

*What is the hope for Epsonia?*

The hope for Epsonia is to have:

- An open-source compeititon server (aka backend)
in order for people to host their own competitions without their own backend,
where they must only worry about creating the image.

- A website for creating images visually, through drag n' drop or something
of that sort.

- A bootstrap tool which takes in your `checks.json` file and will turn it into
a bash script which will *bootstrap* (sets up the vulnerabilities *for*)
your VM image

- Windows & Windows server versions of Epsonia. Currently, I (matees/maytees)
can only use Linux well for Cyberpatriot, so this will be added with outside
help.

## Different components of Epsonia

- Checks
    -   *Good Jobs* - Completed Checks
    -   *Penalties* - Checks which were completed, but then uncompleted
- Hidden Penalties
    - Hidden Penalties are things which you do not want the user
    to tamper with. For example, if you have a user: `james`, and
    this user should exist, and initially exists, then you do not want
    the user to do anything with him, although if the user did, you would
    give them a penalty.

## Rules to Contributing

- Make sure that there are no warnings/errros when running `cargo clippy`.
- Also make sure you run `cargo fmt` before every commit or pull request.
- When Creating a new pull request, make sure to give a good description of your
change.

Other than those 3 "rules", atm I don't care what you do.

Thanks for deciding to contribute to Epsonia!
