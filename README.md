# Secret Network Smart Contract Gas Price Study
A study of different algorithm implementations and their gas impacts.

## Running
To build the contract do
```shell
make release
# or
make debug
```

To run the integration tests you will then need to start the server in another terminal
```shell
make server-start
```

Then you must connect to the container
```shell
make server-connect
```

Whilst inside the container you will have to run
```shell
cd code
make integration-tests
```

After running the integration tests, you will find json files describing runtime information in `./integration-testing`

## Gas Price Analysis

### Permit Contract

The idea of this contract is to show via handleMsgs the impact a permit can have on the network.

The following is taken straight from the `integration-testing` package
```
Set viewing key average gas: 23493
Get viewing key average gas: 22645
Validate permit average gas: 800001
Blocking permit average gas: 22924
```

Validating a permit is ~35x more expensive than simply using a viewing key. While permits are very useful users 
since permits are created offline, if abused they can be detrimental to the nodes used for querying data.

A good UX solution to this should be normalizing user created viewing keys instead of the Keplr default of 
automatically generating them for you.
