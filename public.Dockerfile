FROM node:18-alpine AS builder

WORKDIR /app

# Copy the package.json and package-lock.json from /public directory
COPY public/package.json public/package-lock.json ./

RUN npm ci --quiet

COPY public/ .

RUN npm run build

ENV PORT=80
ENV HOST=0.0.0.0
ENV PUBLIC_URL=https://findog.buttercrab.net
ENV REACT_APP_PUBLIC_URL=https://findog.buttercrab.net

CMD ["npm", "run", "start"]
