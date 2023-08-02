describe('Governor', function () {
  beforeEach(async function () {
  //   
  })
    
  it('deployment check', async function () {
    //
  })

  it('nominal workflow', async function () {
    //
  })

  it('send ethers', async function () {
    //
  })

  describe('vote with signature', function () {
    afterEach('no other votes are cast for proposalId', async function () {
    // 
    })

    it('votes with an EOA signature', async function () {
    // 
    })

    it('votes with a valid EIP-1271 signature', async function () {
    // 
    })

    afterEach('no other votes are cast', async function () {
    //  
    })
  })

  describe('should revert', function () {
    describe('on propose', function () {
      it('if proposal already exists', async function () {
        // 
      })
    })

    describe('on vote', function () {
      it('if proposal does not exist', async function () {
        //
      })

      it('if voting has not started', async function () {
      //
      })

      it('if support value is invalid', async function () {
      //
      })

      it('if vote was already casted', async function () {
        //
      })

      it('if voting is over', async function () {
        //
      })
    })

    describe('on vote by signature', function () {
      beforeEach(async function () {
        //
      })

      it('if signature does not match signer', async function () {
        //
      })

      it('if vote nonce is incorrect', async function () {
        //
      })
    })

    describe('on execute', function () {
      it('if proposal does not exist', async function () {
        //
      })

      it('if quorum is not reached', async function () {
        //
      })

      it('if score not reached', async function () {
        //
      })

      it('if voting is not over', async function () {
        //
      })

      it('if receiver revert without reason', async function () {
        //
      })

      it('if receiver revert with reason', async function () {
        //
      })

      it('if proposal was already executed', async function () {
        //
      })
    })
  })

  describe('state', function () {
    it('Unset', async function () {
      //
    })

    it('Pending & Active', async function () {
      //
    })

    it('Defeated', async function () {
      //
    })

    it('Succeeded', async function () {
      //
    })

    it('Executed', async function () {
      //
    })
  })

  describe('cancel', function () {
    describe('internal', function () {
      it('before proposal', async function () {
        //
      })

      it('after proposal', async function () {
        //
      })

      it('after vote', async function () {
        //
      })

      it('after deadline', async function () {
        //
      })

      it('after execution', async function () {
        //
      })
    })

    describe('public', function () {
      it('before proposal', async function () {
        //
      })

      it('after proposal', async function () {
        //
      })

      it('after proposal - restricted to proposer', async function () {
        //
      })

      it('after vote started', async function () {
        //
      })

      it('after vote', async function () {
        //
      })

      it('after deadline', async function () {
        //
      })

      it('after execution', async function () {
        //
      })
    })
  })

  describe('proposal length', function () {
    it('empty', async function () {
    //
    })

    it('mismatch #1', async function () {
    //
    })

    it('mismatch #2', async function () {
      //
    })

    it('mismatch #3', async function () {
      //
    })
  })

  describe('frontrun protection using description suffix', function () {
    describe('without protection', function () {
      describe('without suffix', function () {
        it('proposer can propose', async function () {
          //
        })

        it('someone else can propose', async function () {
        //
        })
      })

      describe('with different suffix', function () {
        beforeEach(async function () {
          //
        })

        it('proposer can propose', async function () {
          //
        })

        it('someone else can propose', async function () {
          //
        })
      })

      describe('with proposer suffix but bad address part', function () {
        beforeEach(async function () {
          //
        })

        it('propose can propose', async function () {
          //
        })

        it('someone else can propose', async function () {
          //
        })
      })
    })

    describe('with protection via proposer suffix', function () {
      beforeEach(async function () {
        //
      })

      it('proposer can propose', async function () {
        //
      })

      it('someone else cannot propose', async function () {
        //
      })
    })
  })

  describe('onlyGovernance updates', function () {
    it('setVotingDelay is protected', async function () {
      //
    })

    it('setVotingPeriod is protected', async function () {
      //
    })

    it('setProposalThreshold is protected', async function () {
      //
    })

    it('can setVotingDelay through governance', async function () {
      // 
    })

    it('cannot setVotingPeriod to 0 through governance', async function () {
      //
    })

    it('can setProposalThreshold to 0 through governance', async function () {
      // 
    })
  })
})

  