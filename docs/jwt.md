# Структура access-токена

```json
{
  "user_id": "string",
  "phone": "string",
  "telegram_id": "string",
  "first_name": "string",
  "last_name": "string",
  "patronymic": "string",
  "user_types": [
    {
      "type": "customer",
      "id": "string",
      "organization_id": "string"
    },
    {
      "type": "employee",
      "roles": ["owner", "manager", "master"],
      "organization_id": "string",
      "master_branch_id": "string" | null
    }
  ]
}
```
