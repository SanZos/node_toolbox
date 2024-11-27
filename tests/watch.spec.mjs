import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import test, { suite } from 'node:test';
import assert from 'node:assert/strict';
import { clearWatch, watch } from '../index.js';

suite('Watch', () => {
    test('watch a file', () => {
        writeFileSync('____test.json', JSON.stringify({ test: 'a' }));
        let handler = watch('____test.json', (err, data) => {
            assert.equal(err, null);
            assert.equal(data, "____test.json Changed");
            clearWatch(handler);
            rmSync('____test.json');
        });
        setTimeout(() => writeFileSync('____test.json', JSON.stringify({ test: 'a' })), 50);
    });

    test('watch a dir', () => {
        mkdirSync('____testDir');
        writeFileSync('____testDir/test.json', JSON.stringify({ test: 'a' }));
        let i = 0;
        let handler = watch('____testDir', (err, data) => {
            assert.equal(err, null);
            if (i === 0) {
                assert.equal(data, "____testDir/z.json Created");
                rmSync('____testDir/z.json');
            }
            if (i === 1) {
                assert.equal(data, "____testDir/z.json Deleted");
                clearWatch(handler);
                rmSync('____testDir', { recursive: true, force: true });
            }
            i++;
        });
        writeFileSync('____testDir/z.json', JSON.stringify({ test: 'ab' }));
    });

    test('error file not exist', (_, done) => {
        let handler = watch('-1', (err, data) => {
            assert.equal(err.message, "Directory doesn't exist");
            done();
        });
        assert.equal(handler, false);
    });

    test('delete the only watched file', () => {
        writeFileSync('____test.json2', JSON.stringify({ test: 'a' }));
        watch('____test.json2', (err, data) => {
            assert.equal(err, null);
            assert.equal(data, "____test.json2 Deleted");
        });
        setTimeout(() => rmSync('____test.json2'), 100);
    });
});