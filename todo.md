###### thoughts on design

To add branching I'll probably need to add internal `Branch` struct that will hold nodes.
So then when "choice" is made, nodes from `Branch` will be appended to `Besida` struct.

Usually "dialogue tree" is implemented with directed graph - dialogue only moves "forward",
but sometimes needs to be merged into another branch.

So I thought maybe we could to something like this:

> Dialogue from [this](https://en.wikipedia.org/wiki/Dialogue_tree#/media/File:Dialog_tree_example.svg) picture

```
!--- Branching dialogue with jumps ---

Man:
    You don't look like you're from around here.

You:
    > I've lived here all my life! -> mr_bowler_branch

    > I came here from Newton.

        Man:
            Newton, eh? I heard there's trouble brewing down there.

        You:
            > Did I say Newton? I'm actually from Springville. -> mr_bowler_branch

            > I haven't heard about any trouble. -> dont_worry


--- mr_bowler_branch ---

Man:
    Oh really? Then you must know Mr. Bowler.

You:
    > Mr. Bowler is a good friend of mine!
        Man:
            You liar! There ain't no Mr. Bowler, I made him up!

    > Who? -> dont_worry


--- dont_worry ---

Man:
Don't you worry about it. Say, do you have something to eat? I'm starving.
```

Here branch is defined by `--- {branch_name} ---` and placing `!` in front makes
it an entry point (else default to first branch).

`-> {branch_name}` is a jump which will append nodes from branch to current `Vec` of nodes. `>` is an option.

I know... It's _very_ similar to what Ink does, and I found out about it later than I wanted to.
But it made me somewhat happy that I made the design choice similar to theirs.
I'm not sure if they support "event stream" similar to one seen in
[Hyperbolica devlog](https://www.youtube.com/watch?v=DlL_20x0QH8) (by the way, my main inspiration)
and I'm not feeling like learning Ink anyways.

Main idea of this project is to create an easy to write file format which can be parsed from wherever.
