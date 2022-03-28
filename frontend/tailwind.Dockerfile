FROM node:17-alpine3.15

WORKDIR /opt/app

RUN npm install -g \
     tailwindcss@latest
    # @tailwindcss/typography \
    # @tailwindcss/forms \
    # @tailwindcss/aspect-ratio \
    # @tailwindcss/line-clamp \
    # postcss@latest \
    # autoprefixer@latest

CMD [ "/bin/sh", "-c", "npx tailwindcss -c tailwind.config.js -i tailwind.in.css -o tailwind.css --minify --watch" ]