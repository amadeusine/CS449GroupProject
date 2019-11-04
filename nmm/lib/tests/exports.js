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

    it('can call the poll method defined on Manager in Rust', function() {
        var mngr = new Manager();
        var result = mngr.poll();
        assert.equal(result.handle, 'Ok');
        assert.equal(result.trigger, 'None');
        assert.equal(result.board.length, 24);
    });
    it('can call the get_board method defined on Manager in Rust', function() {
        var mngr = new Manager();
        var result = mngr.get_board();
        assert.equal(result.length, 24);
        assert.equal(result["A1"], 'None');
    });
});


