module.exports = {
    root: true,
    env: { browser: true, es2020: true },
    extends: [
        'eslint:recommended',
        'airbnb',
        'plugin:react/recommended',
        'plugin:react/jsx-runtime',
        'plugin:react-hooks/recommended',
        'eslint-config-prettier',
        'plugin:import/recommended',
    ],
    ignorePatterns: ['dist', '.eslintrc.cjs', 'emu-wasm'],
    parserOptions: { ecmaVersion: 'latest', sourceType: 'module' },
    settings: {
        react: { version: '18.2' },
        'import/resolver': {
            node: {
                extensions: ['.js', '.jsx'],
            },
        },
    },
    plugins: ['react-refresh', 'import'],
    rules: {
        'react/jsx-no-target-blank': 'off',
        'react-refresh/only-export-components': [
            'warn',
            { allowConstantExport: true },
        ],
        'import/newline-after-import': ['error', { count: 1 }],
        'react/prop-types': 0,
        'react/function-component-definition': [
            2,
            {
                namedComponents: 'arrow-function',
                unnamedComponents: 'arrow-function',
            },
        ],
        'no-param-reassign': ['error', { props: false }],
    },
}
