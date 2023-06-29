FROM node:18-alpine AS builder

WORKDIR /app

# Copy the package.json and package-lock.json from /public directory
COPY public/package.json public/package-lock.json ./

RUN npm ci --quiet

COPY public/ .

RUN npm run build

CMD ["npm", "run", "start", "--", "-p", "80"]
