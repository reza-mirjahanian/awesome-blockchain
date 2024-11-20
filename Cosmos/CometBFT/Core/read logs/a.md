Next follows a standard block creation cycle, where we enter a new round, propose a block, receive more than 2/3 of prevotes, then precommits and finally have a chance to commit a block. For details, please refer to [Byzantine Consensus Algorithm](https://github.com/cometbft/cometbft/blob/v0.38.x/spec/consensus/consensus.md).

```
I[10-04|13:54:30.393] enterNewRound(91/0). Current: 91/0/RoundStepNewHeight module=consensus
I[10-04|13:54:30.393] enterPropose(91/0). Current: 91/0/RoundStepNewRound module=consensus
I[10-04|13:54:30.393] enterPropose: Our turn to propose            module=consensus proposer=125B0E3C5512F5C2B0E1109E31885C4511570C42 privValidator="PrivValidator{125B0E3C5512F5C2B0E1109E31885C4511570C42 LH:90, LR:0, LS:3}"
I[10-04|13:54:30.394] Signed proposal                              module=consensus height=91 round=0 proposal="Proposal{91/0 1:21B79872514F (-1,:0:000000000000) {/10EDEDD7C84E.../}}"
I[10-04|13:54:30.397] Received complete proposal block             module=consensus height=91 hash=F671D562C7B9242900A286E1882EE64E5556FE9E
I[10-04|13:54:30.397] enterPrevote(91/0). Current: 91/0/RoundStepPropose module=consensus
I[10-04|13:54:30.397] enterPrevote: ProposalBlock is valid         module=consensus height=91 round=0
I[10-04|13:54:30.398] Signed and pushed vote                       module=consensus height=91 round=0 vote="Vote{0:125B0E3C5512 91/00/1(Prevote) F671D562C7B9 {/89047FFC21D8.../}}" err=null
I[10-04|13:54:30.401] Added to prevote                             module=consensus vote="Vote{0:125B0E3C5512 91/00/1(Prevote) F671D562C7B9 {/89047FFC21D8.../}}" prevotes="VoteSet{H:91 R:0 T:1 +2/3:F671D562C7B9242900A286E1882EE64E5556FE9E:1:21B79872514F BA{1:X} map[]}"
I[10-04|13:54:30.401] enterPrecommit(91/0). Current: 91/0/RoundStepPrevote module=consensus
I[10-04|13:54:30.401] enterPrecommit: +2/3 prevoted proposal block. Locking module=consensus hash=F671D562C7B9242900A286E1882EE64E5556FE9E
I[10-04|13:54:30.402] Signed and pushed vote                       module=consensus height=91 round=0 vote="Vote{0:125B0E3C5512 91/00/2(Precommit) F671D562C7B9 {/80533478E41A.../}}" err=null
I[10-04|13:54:30.404] Added to precommit                           module=consensus vote="Vote{0:125B0E3C5512 91/00/2(Precommit) F671D562C7B9 {/80533478E41A.../}}" precommits="VoteSet{H:91 R:0 T:2 +2/3:F671D562C7B9242900A286E1882EE64E5556FE9E:1:21B79872514F BA{1:X} map[]}"
I[10-04|13:54:30.404] enterCommit(91/0). Current: 91/0/RoundStepPrecommit module=consensus
I[10-04|13:54:30.405] Finalizing commit of block with 0 txs        module=consensus height=91 hash=F671D562C7B9242900A286E1882EE64E5556FE9E root=E0FBAFBF6FCED8B9786DDFEB1A0D4FA2501BADAD
I[10-04|13:54:30.405] Block{
  Header{
    ChainID:        test-chain-3MNw2N
    Height:         91
    Time:           2017-10-04 13:54:30.393 +0000 UTC
    NumTxs:         0
    LastBlockID:    F15AB8BEF9A6AAB07E457A6E16BC410546AA4DC6:1:D505DA273544
    LastCommit:     56FEF2EFDB8B37E9C6E6D635749DF3169D5F005D
    Data:
    Validators:     CE25FBFF2E10C0D51AA1A07C064A96931BC8B297
    App:            E0FBAFBF6FCED8B9786DDFEB1A0D4FA2501BADAD
  }#F671D562C7B9242900A286E1882EE64E5556FE9E
  Data{

  }#
  Commit{
    BlockID:    F15AB8BEF9A6AAB07E457A6E16BC410546AA4DC6:1:D505DA273544
    Precommits: Vote{0:125B0E3C5512 90/00/2(Precommit) F15AB8BEF9A6 {/FE98E2B956F0.../}}
  }#56FEF2EFDB8B37E9C6E6D635749DF3169D5F005D
}#F671D562C7B9242900A286E1882EE64E5556FE9E module=consensus
I[10-04|13:54:30.408] Executed block                               module=state height=91 validTxs=0 invalidTxs=0
I[10-04|13:54:30.410] Committed state                              module=state height=91 txs=0 hash=E0FBAFBF6FCED8B9786DDFEB1A0D4FA2501BADAD
I[10-04|13:54:30.410] Recheck txs                                  module=mempool numtxs=0 height=91

```