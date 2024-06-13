/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
      './src/**/*{rs,html,js}',
      './templates/**/*{rs,html,js}',
      './static/**/*{go,html,js}',
    ],
    theme: {
        extend: {
            colors: {
                'primary': '#E51636',
                'error': '#750c05',
                'gray': {
                    100: '#F7F7F7',
                    500: '#999999',
                    900: '#333333',
                }
            },
        },
    },
    plugins: [],
  };
  