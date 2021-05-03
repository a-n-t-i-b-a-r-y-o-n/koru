TODO
====

## Features
_Specific, unimplemented things that other people or future me might want_

- [ ] __Find Remote__  
My device at home doesn't have this, so I don't know how to begin testing.  
I'll back-burner implementing this for now.
  
- [ ] __Search__  
Don't really use this myself, so just haven't done it yet.
  
- [ ] __URLs/Deep Linking__  
This is probably next up on the to-do.
  
- [ ] __App Icons__  
Still mulling over implementation details.

## Incomplete
_Areas of the code that could use improvement_


- [ ] __Errors__  
Device-level errors are all Strings instead of proper Error types.
  

- [ ] __Magic packets__
* Currently using a crate for this, should at least audit it  
* Failure is handled in `client.waking_post()` with the `IM_A_TEAPOT` StatusCode for now

## Brainstorming
_Half-baked feature ideas, avenues to explore, potential ~~attack~~ fun vectors_

#### Private listening
Seems like an incredibly valuable feature-add, though AFAIUI the implementation is proprietary.  
Will continue... research 😈

#### Info keys to explore
* `supports-ecs-microphone`  
  Is this a mic that's **part of** or **connecting to** the device?
* `supports-ecs-textedit`  
  This might be fun from a security perspective.