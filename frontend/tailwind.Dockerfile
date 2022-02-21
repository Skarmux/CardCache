FROM node:17-alpine3.14

WORKDIR /app

RUN npm install \
    tailwindcss@latest \
    # @tailwindcss/typography@latest \
    # @tailwindcss/forms@latest \
    # @tailwindcss/aspect-ratio@latest \
    # @tailwindcss/line-clamp@latest \
    postcss@latest \
    autoprefixer@latest

#WORKDIR /

#CMD [ "npx", "tailwindcss", "-c", "app/tailwind.config.js", "-i", "app/tailwind.in.css", "-o", "app/tailwind.css", "--minify", "--watch", "app/**"]
