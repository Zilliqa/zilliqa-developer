var assert = require('assert');
var fs = require('fs');
var path = require('path');

// Initialize testing library
const ZilTest = require('zilliqa-testing-library').ZilTest;
const Test = new ZilTest();

// Read contract file
const contract = fs.readFileSync(path.join(__dirname, '../<%= projectName %>.scilla'), 'utf8');

describe('Run <%= projectName %> tests...', function () {

    it('should load Zilliqa Provider', async function () {
        assert(Test.zilliqa !== undefined);
    });

});
