---
tags:
  - error
  - transaction
---

# Transaction errors and troubleshooting

## Why does my transaction error?

This could be for a number of issues. Firstly, read and understand the error.

Lets take the below example where a user is trying to interface with Zilswap dex.

![Docusaurus](/img/side-knowledge/troubleshooting/troubleshooting-1.png)

Firstly, we can see the transition with the passed variables variables, and below that we can see the exception stack trace.

The error thrown reads ```Exception thrown: (Message [(_exception : (String "RequestedRatesCannotBeFulfilled"))]) :1```

If errors are well defined, it may be easy to spot what the problem is, in our case we do not know what ```RequestedRatesCannotBeFulfilled``` means.

This is a contract specific error and we will need to look at the Zilswap Dex contract to learn more.

Once we've searched the Zilswap Dex contract for the phrase ```RequestedRatesCannotBeFulfilled``` we find it posted in three places.

The first two do not concern us, as these are around ```AddLiquidity``` and ```RemoveLiquidity```.

The third and final place its mentioned is in the library function ```resultFor```.

We can find ```resultFor``` used in most of the swap transitions including our transition call ```SwapExactTokensForZIL```.

We have traced the error back to this particular logical block.

```ocaml
  let within_limits = withinLimits amount maybe_limit_amount exact_side in
  match within_limits with
  | False =>
     let e = "RequestedRatesCannotBeFulfilled" in Error e
  | True =>
     Result pool amount
  end 
```

We can further extrapolate on the intermediately proceeding code that this logical block is dealing with swapping user tokens against the liquidity pool - this error is returned when either the price moves and the amount of slippage determines that the trade calculation would not have enough tokens to proceed and therefore throws this error.

## Typical Errors

A non exhausted list of errors and some tips for working a resolution.

### Failed to parse json

Input passed wasn't in the expected shape, print the request and parameters and check what's being passed and what types are present.

### No parameter found matching message entry xxx

A transition or procedure is expecting a parameter which is missing, is your call passing this data in the expected order? This error can also be thrown on callbacks.

### "JSON_OUTPUT_CORRUPTED","CHECKER_FAILED"

Check your initialisation parameters are valid.

### To Addr checksum wrong

Base16 addresses use uppercase and lowercase letters to be used as a checksum, the address still works as a public key in all lowercase or uppercase but then the checksum is invalid - try making the string lowercase.

### Ran out of gas after evaluating statement

Increase your gas limit.

### xxx type not allowed in JSON file

Check the types in your request to what's on the contract.

## Further reading

[Zilswap Dex contract - L165](https://viewblock.io/zilliqa/address/zil1gkwt95a67lnpe774lcmz72y6ay4jh2asmmjw6u?tab=code)