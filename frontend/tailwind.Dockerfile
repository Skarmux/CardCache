FROM node:17-alpine3.15

WORKDIR /app/frontend

RUN npm install -g \
     tailwindcss@latest \
    # @tailwindcss/typography \
    # @tailwindcss/forms \
    # @tailwindcss/aspect-ratio \
    # @tailwindcss/line-clamp \
    # postcss@latest \
    # autoprefixer@latest
#
# CMD in docker does not run --watch option in a loop,
# the command is being executed with shell context from within docker-compose
#
# CMD ["npx", "tailwindcss", "-i", "tailwind.in.css", "-o", "tailwind.css", "-c", "tailwind.config.js", "--minify", "--watch"]