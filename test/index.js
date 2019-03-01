// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape

// use this when running off your own dev instance of the nodejs_conductor
// const { Config, Conductor, Scenario } = require('../../holochain-rust/nodejs_conductor')
const { Config, Conductor, Scenario } = require('../../../rust/nodejs_conductor')

// for live use
//const { Config, Conductor, Scenario } = require("@holochain/holochain-nodejs")

Scenario.setTape(require('tape'))

const dnaPath = "./dist/simple-app.dna.json"

const dna = Config.dna(dnaPath, 'app-spec')

const agentAlice = Config.agent("alice")

const instanceAlice = Config.instance(agentAlice, dna)

const scenario = new Scenario([instanceAlice])


scenario.runTape('test of share and get', async (t, { alice }) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
    const addr = alice.call("simple", "share_item", {"item" : {"content":"sample content"}})

    const result = alice.call("simple", "get_item", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'item', '{"content":"sample content"}' ] } })

})
