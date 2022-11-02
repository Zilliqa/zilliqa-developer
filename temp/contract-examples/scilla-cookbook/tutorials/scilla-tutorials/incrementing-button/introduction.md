---
sidebar_position: 1
---

# Introduction

We are going to write a Scilla contract from scratch.

You don't need any previous blockchain experience, but general programming experience will be useful.

Best practices and developer thoughts will be present to help you start to think like a chain developer.

## IncrementingButton

IncrementingButton is a walkthrough tutorial of the basic concepts of writing Scilla contract code with the following objectives.

* Create a Scilla contract which models a button that can be pressed by anyone.
  * If you are the recent button presser you may not press the button again.
  * When the button is pressed: Increment a counter and set the caller of the button press to be the most recent address.  
  * The owner of the button contract has the ability to reset the counter to zero.
