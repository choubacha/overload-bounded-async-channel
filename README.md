# Overloading a bounded async channel

Due to the fact that the the bounds on a futures channel are `n + s`. Where
`n` is the specified bounds and `s` is the number of senders. If we clone the
sender and don't get it "back" then we infinitely expand the channels capacity
for each new sender.
