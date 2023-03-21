/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx,vue}",
    ],
    theme: {
        extend: {
            colors: {
                'sf': {
                    '50': '#eefcfd',
                    '100': '#d3f3fa',
                    '200': '#ace6f5',
                    '300': '#73d3ed',
                    '400': '#33b6dd',
                    '500': '#1798c1',
                    '600': '#167aa4',
                    '700': '#196485',
                    '800': '#1d526d',
                    '900': '#1c455d',
                },
            }
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}
