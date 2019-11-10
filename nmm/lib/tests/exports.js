var addon = require('../../native');
var assert = require('chai').assert;

const Manager = addon.Manager;

describe('Neon Exported Objects', function () {
    it('return Manager built in Rust', function () {
        assert.isFunction(addon.Manager);
    });

    it('construct a Manager built in Rust', function() {
        var mngr = new Manager();
        assert(mngr instanceof Manager);
    });

    it('can call get_curr_state method defined on Manager in Rust', function() {
        var mngr = new Manager();
        var result = mngr.get_curr_state();
        assert.equal(result.handle, 'Ok');
        assert.equal(result.trigger, 'None');
        assert.equal(result.board.length, 24);
    });
    it('can call NAIVE poll() method defined on Manager in Rust', function() {
        var mngr = new Manager();
        var options = {"user": 1, "opponent": 2, "agent": "human", "position": ["A", "1"]};
        var type = "Menu";
        var result = mngr.poll(type, options);
        assert.equal(result, 'Ya did it!');
    });
    it('can call the get_board method defined on Manager in Rust', function() {
        var mngr = new Manager();
        var result = mngr.get_board();
        assert.equal(result.length, 24);
        assert.equal(result["A1"], 'None');
    });
});


