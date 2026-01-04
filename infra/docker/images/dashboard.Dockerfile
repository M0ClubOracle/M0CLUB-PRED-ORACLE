
FROM node:20-slim as builder
WORKDIR /app
COPY ../../services/dashboard/package.json ../../services/dashboard/tsconfig.json ../../services/dashboard/next.config.js /app/
COPY ../../services/dashboard/src /app/src
COPY ../../services/dashboard/public /app/public
RUN npm install
RUN npm run build

FROM node:20-slim
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app/.next/standalone /app/
COPY --from=builder /app/.next/static /app/.next/static
COPY --from=builder /app/public /app/public
EXPOSE 3000
CMD ["node", "server.js"]
