<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# Как в openapi (и utoipa в частности) принято документировать работу с авторизацией через JWT?

В OpenAPI и utoipa принято документировать JWT авторизацию с помощью схем безопасности (Security Schemes). Вот основные подходы:

## Схема безопасности Bearer Token

В OpenAPI JWT токены обычно документируются как Bearer Token схема безопасности. В utoipa это реализуется следующими способами:[^1_1][^1_2]

### Глобальная схема безопасности

Наиболее распространенный подход - определить схему безопасности на глобальном уровне с помощью trait `Modify`:[^1_3]

```rust
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(modifiers(&SecurityAddon))]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
```


### Применение схемы к эндпоинтам

После определения схемы безопасности ее можно применить к конкретным эндпоинтам с помощью атрибута `security` в макросе `#[utoipa::path]`:[^1_4]

```rust
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "Authenticated User", body = UserResponseDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    security(
        ("api_jwt_token" = [])
    )
)]
async fn get_authenticated_user() -> Result<UserResponseDto, Error> {
    // реализация эндпоинта
}
```


### Полный пример для Spring Boot

Для Java с использованием Spring Boot и OpenAPI 3.0 схема настраивается следующим образом:[^1_2]

```java
@Configuration
public class OpenAPI30Configuration {
    @Bean
    public OpenAPI customizeOpenAPI() {
        final String securitySchemeName = "bearerAuth";
        return new OpenAPI()
            .addSecurityItem(new SecurityRequirement()
                .addList(securitySchemeName))
            .components(new Components()
                .addSecuritySchemes(securitySchemeName, new SecurityScheme()
                    .name(securitySchemeName)
                    .type(SecurityScheme.Type.HTTP)
                    .scheme("bearer")
                    .bearerFormat("JWT")));
    }
}
```


## Ключевые моменты

1. **Тип схемы**: Используется `SecurityScheme::Http` с `HttpAuthScheme::Bearer`[^1_5][^1_6]
2. **Формат токена**: Указывается `bearer_format("JWT")` для ясности[^1_5]
3. **Название схемы**: Обычно используются названия типа `"bearerAuth"`, `"api_jwt_token"` или `"token"`[^1_2][^1_4]
4. **Применение**: Схема применяется через атрибут `security` с пустым массивом разрешений `[]`[^1_4]

## Интеграция с Swagger UI

После правильной настройки схемы безопасности в Swagger UI появится:

- Кнопка **"Authorize"** для ввода JWT токена[^1_4]
- Значки замка на защищенных эндпоинтах
- Возможность тестировать защищенные API с токеном[^1_2]

Такой подход обеспечивает стандартную документацию JWT авторизации, совместимую с OpenAPI 3.0/3.1 спецификацией и позволяет легко тестировать защищенные API через Swagger UI.

<div style="text-align: center">⁂</div>

[^1_1]: https://swagger.io/docs/specification/v3_0/authentication/bearer-authentication/

[^1_2]: https://www.baeldung.com/openapi-jwt-authentication

[^1_3]: https://docs.rs/utoipa/latest/utoipa/trait.Modify.html

[^1_4]: https://codevoweb.com/add-swagger-ui-redoc-and-rapidoc-to-a-rust-api/

[^1_5]: https://www.libelektra.org/ftp/permaplant/doc/dev/cargodoc/utoipa/openapi/security/enum.SecurityScheme.html

[^1_6]: https://apatisandor.hu/blog/production-ready-openapi/

[^1_7]: https://babok-school.ru/blog/openapi-bearer-token-ishop-example/

[^1_8]: https://swagger.io/docs/specification/v3_0/authentication/

[^1_9]: https://thecodebuzz.com/jwt-authorization-token-swagger-open-api-asp-net-core-3-0/

[^1_10]: https://learn.microsoft.com/en-us/aspnet/core/security/authentication/configure-jwt-bearer-authentication?view=aspnetcore-9.0

[^1_11]: https://docs.rs/crate/utoipa/1.0.0/source/src/lib.rs

[^1_12]: https://github.com/juhaku/utoipa

[^1_13]: https://github.com/juhaku/utoipa/issues/919

[^1_14]: https://crates.io/crates/axum-jwt-sessions

[^1_15]: https://docs.rs/crate/utoipa/1.0.0/source/tests/modify_test.rs

[^1_16]: https://users.rust-lang.org/t/adding-header-to-a-http-request-with-swagger/102322

[^1_17]: https://stackoverflow.com/questions/78070826/how-do-i-generate-the-openapi-schema-for-an-optional-query-parameter-in-utoipa

[^1_18]: https://users.rust-lang.org/t/how-to-create-common-http-response-examples-in-utoipa-swagger/111368

[^1_19]: https://testdriven.io/blog/fastapi-jwt-auth/


---

# Как описываются роли в OpenAPI и utoipa?

В OpenAPI и utoipa роли пользователя обычно описываются через:

1. **Стандартные механизмы — scopes (области) в схемах безопасности**
В OpenAPI для OAuth2 схема безопасности может включать scopes, которые технически используются для авторизации и часто отображают "роли". Например:

```yaml
securitySchemes:
  my_auth:
    type: oauth2
    flows:
      authorizationCode:
        scopes:
          read:items: "Чтение"
          edit:items: "Редактирование"
```

На уровне эндпоинта можно указать необходимые scope:

```yaml
security:
  - my_auth: ["read:items", "edit:items"]
```

В utoipa аналогично:

```rust
#[utoipa::path(
    get,
    path = "/pet",
    security(
       ("my_auth" = ["read:items", "edit:items"])
    )
)]
fn get_pet() -> Pet {...}
```

Таким образом, в документации видно, какие "роли/пермишены" нужны для доступа к эндпоинту.[^2_1][^2_2]
2. **Кастомные или расширенные поля (расширения x-roles, x-permissions)**
В классическом OpenAPI роли используют либо scopes, либо добавляют кастомные поля (например, `x-roles` или `x-ibm-zcon-roles-allowed`). Пример для IBM z/OS Connect:

```yaml
openapi: 3.0.0
x-ibm-zcon-roles-allowed:
  - Staff
paths:
  /employees:
    get:
      x-ibm-zcon-roles-allowed:
        - Manager
        - HR
```

То есть роли можно указывать для всего API или для отдельных методов через кастомные расширения.[^2_3]
3. **Документация/Описание**
Если механизм авторизации не подразумевает scopes, требования по ролям можно документировать в описании эндпоинта (`description`), либо ссылкой на отдельную таблицу ролей.

**Вывод:**

- В OpenAPI и utoipa роли отражаются либо стандартными scope (чаще — для OAuth2), либо через кастомные поля-расширения, либо через поле `security` для protected endpoint'ов.
- Для JWT авторизации обычно не предусмотрено scopes out-of-the-box, но требования к ролям можно указать в описании или кастомном поле для совместимости с потребителями API документации.

<div style="text-align: center">⁂</div>

[^2_1]: https://docs.rs/utoipa/latest/utoipa/attr.path.html

[^2_2]: https://www.libelektra.org/ftp/permaplant/doc/mr/cargodoc/utoipa/openapi/security/struct.SecurityRequirement.html

[^2_3]: https://www.ibm.com/docs/en/cloud-paks/z-modernization-stack/2023.4?topic=authorization-how-define-roles

[^2_4]: https://swagger.io/specification/

[^2_5]: https://stackoverflow.com/questions/76153735/how-to-display-required-user-roles-in-for-openapi-ui

[^2_6]: https://learn.openapis.org/specification/security.html

[^2_7]: https://www.youtube.com/watch?v=MXP76cBES6E

[^2_8]: https://github.com/OAI/OpenAPI-Specification/issues/2379

[^2_9]: https://github.com/juhaku/utoipa

[^2_10]: https://docs.rs/utoipa/latest/i686-pc-windows-msvc/utoipa/openapi/security/index.html

[^2_11]: https://github.com/juhaku/utoipa/discussions/786

