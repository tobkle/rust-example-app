/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '../web-pages/**/*.rs',
  ],
  safelist: [
    'bg-base-100',
    'bg-base-200',
    'bg-base-300',
    'border-base-300',
    'text-base-content',
  ],
};
