use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// Импортируем необходимые модули

// Определяем функцию-обработчик маршрута "/"
#[get("/")]
async fn index() -> impl Responder {
    // Возвращаем HTTP-ответ с телом "Hello, world!"
    HttpResponse::Ok().body("Hello, world!")
}

// Определяем функцию main(), которая будет запускать сервер
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Создаем экземпляр HttpServer, который будет обрабатывать запросы
    HttpServer::new(|| {
        // Создаем новый экземпляр App, который будет содержать маршруты нашего приложения
        App::new()
            // Регистрируем наш маршрут "/"
            .service(index)
    })
    // Привязываем сервер к локальной адресе 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    // Запускаем сервер
    .run()
    // Ожидаем завершения работы сервера
    .await
}