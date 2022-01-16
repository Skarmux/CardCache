docker run -p 7878:5432 -d postgres:9.6.12;
cd backend; make dev
cd ../frontend; make web