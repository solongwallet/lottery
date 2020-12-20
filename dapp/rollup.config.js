import babel from 'rollup-plugin-babel';
import builtins from 'rollup-plugin-node-builtins';
import commonjs from 'rollup-plugin-commonjs';
import copy from 'rollup-plugin-copy';
import globals from 'rollup-plugin-node-globals';
import json from 'rollup-plugin-json';
import nodeResolve from 'rollup-plugin-node-resolve';
import replace from 'rollup-plugin-replace';
import {terser} from 'rollup-plugin-terser';

const env = process.env.NODE_ENV;

function generateConfig(configType) {
  const config = {
    input: 'src/index.js',
    plugins: [
      json(),
      babel({
        exclude: '**/node_modules/**',
        runtimeHelpers: true,
      }),
      replace({
        'process.env.NODE_ENV': JSON.stringify(env),
      }),
      commonjs(),
      copy({
        targets: [{src: 'module.d.ts', dest: 'lib', rename: 'index.d.ts'}],
      }),
    ],
  };

  switch (configType) {
    case 'browser':
      config.output = [
        {
          file: 'lib/index.iife.js',
          format: 'iife',
          name: 'solonglottery',
          sourcemap: true,
        },
      ];
      config.plugins.push(builtins());
      config.plugins.push(globals());
      config.plugins.push(
        nodeResolve({
          browser: true,
        }),
      );

      if (env === 'production') {
        config.plugins.push(
          terser({
            mangle: false,
            compress: false,
          }),
        );
      }

      break;
    case 'node':
      config.output = [
        {
          file: 'lib/index.cjs.js',
          format: 'cjs',
          sourcemap: true,
        },
        {
          file: 'lib/index.esm.js',
          format: 'es',
          sourcemap: true,
        },
      ];
      config.external = [
        'tweetnacl',
        'bip32',
        'bip39',
        'bs58',
        'crypto',
        '@solana/web3.js',
      ];
      break;
    default:
      throw new Error(`Unknown configType: ${configType}`);
  }

  return config;
}

//export default [generateConfig('node'), generateConfig('browser')];
export default [generateConfig('node')];