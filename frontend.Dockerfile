FROM node:latest

COPY ./src/ /app/src/
COPY ./static/ /app/static/
COPY ["./package.json", "./start.sh", "./vite.config.ts", "tsconfig.json", "tailwind.config.ts","svelte.config.js","postcss.config.cjs","/app/" ] 

WORKDIR /app/

RUN npm install
RUN npm run build

ENTRYPOINT [ "./start.sh" ]