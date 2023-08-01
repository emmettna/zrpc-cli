# ZRPC-CLI
## What is zrpc-cli
- Reflection based grpc client using `grpcurl` under the hood

## Pre-requisite
- [grpcurl](https://github.com/fullstorydev/grpcurl)
- Grpc server providing descriptor(s)
  - [an example can be found at tonic](https://github.com/hyperium/tonic/blob/master/examples/src/reflection/server.rs)

## How to use
1. Run `zrpc-cli` on terminal
2. Input `host`
3. Input `port`
4. Choose service out of printed index
5. Choose method out of printed index
6. Provide json body if needed
   1. If provided json is incorrect, it will suggest a fixed json data(only if it's a simple typo)
7. Check result
8. Choose to repeat or go back to step by index.

## Example on Terminal
```
# Your favorite terminal

> zrpc-cli
---------------------------------------------------
Type Host or `Enter` for "localhost"
> (enter)
---------------------------------------------------
Type Port or `Enter` for "9090"
> 50052
---------------------------------------------------
Select service to proceed
[0] grpc.reflection.v1alpha.ServerReflection
[1] helloworld.Greeter
> 1
---------------------------------------------------
Select function to proceed
[0] helloworld.Greeter.SayHello
> 0
---------------------------------------------------
Type request body
Type 3 new lines in order to finish(`Enter` 3 times)
> name: John
> (enter)
Invalid JSON format. Did you mean this instead?

 => {"name":"John"}

        1: Yes
        2: No

> 1
---------------------------------------------------
Sent request
Server response:
{
  "message": "Hello johnny!"
}
---------------------------------------------------

Press `Enter` if want to repeat the same request. Other wise select which Sequence
    1. Set Host
    2. Set Port
    3. Set Service
    4. Set Function
    5. Set body
    6. Repeat(or Enter)
    -------------------
    7. to exit(or 'exit')

```
- [Used this grpc server for above example](https://github.com/emmettna/sample_tonic_grpc_server)