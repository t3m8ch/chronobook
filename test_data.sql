-- Тестовые данные для ChronoBook
-- Организация "Рога и копыта" с филиалами, мастерами, услугами и бронированиями

-- Очистка существующих данных (в обратном порядке зависимостей)
TRUNCATE TABLE scheduled_notifications CASCADE;
TRUNCATE TABLE notification_templates CASCADE;
TRUNCATE TABLE notification_settings CASCADE;
TRUNCATE TABLE telegram_verify_hashes CASCADE;
TRUNCATE TABLE phone_verify_codes CASCADE;
TRUNCATE TABLE booking_status_change CASCADE;
TRUNCATE TABLE bookings CASCADE;
TRUNCATE TABLE customers CASCADE;
TRUNCATE TABLE day_redefinitions CASCADE;
TRUNCATE TABLE schedule_days CASCADE;
TRUNCATE TABLE timetables CASCADE;
TRUNCATE TABLE services CASCADE;
TRUNCATE TABLE employees CASCADE;
TRUNCATE TABLE branches CASCADE;
TRUNCATE TABLE user_profiles CASCADE;
TRUNCATE TABLE users CASCADE;
TRUNCATE TABLE organizations CASCADE;

-- 1. Создаем организацию
INSERT INTO organizations (id, created_at, updated_at, name, display_name, description) 
VALUES ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'roga-i-kopyta', 'Рога и копыта', 'Сеть салонов красоты премиум-класса с широким спектром услуг');

-- 2. Создаем филиалы
INSERT INTO branches (id, created_at, updated_at, display_name, description, timezone, street, house_number, apartment_number, city, region, country, address_info, organization_id) 
VALUES 
('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Центральный салон', 'Наш главный салон в самом сердце города', 'Europe/Saratov', 'Московская', '12', NULL, 'Саратов', 'Саратовская область', 'Россия', 'Рядом с ТЦ Центральный', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Филиал на Волжской', 'Уютный салон в спальном районе', 'Europe/Saratov', 'Волжская', '45А', '1', 'Саратов', 'Саратовская область', 'Россия', 'Первый этаж жилого дома, отдельный вход', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11');

-- 3. Создаем пользователей
INSERT INTO users (id, created_at, updated_at, phone, telegram_id, phone_verified_at, telegram_verified_at) 
VALUES 
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234567', NULL, NOW(), NULL),
('c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234568', NULL, NOW(), NULL),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234569', NULL, NOW(), NULL),
('c3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234570', NULL, NOW(), NULL),
('c4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234571', NULL, NOW(), NULL),
('c5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234572', NULL, NOW(), NULL),
('c6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234573', NULL, NOW(), NULL),
('c7eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234574', NULL, NOW(), NULL),
('c8eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234575', NULL, NOW(), NULL),
('c9eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234576', NULL, NOW(), NULL),
('ca0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234577', NULL, NOW(), NULL);

-- 4. Создаем профили пользователей
INSERT INTO user_profiles (user_id, created_at, updated_at, first_name, last_name, patronymic) 
VALUES
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Иван', 'Петров', 'Сергеевич'),
('c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Анна', 'Смирнова', 'Александровна'),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Мария', 'Иванова', 'Петровна'),
('c3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Елена', 'Козлова', 'Викторовна'),
('c4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Ольга', 'Новикова', 'Андреевна'),
('c5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Татьяна', 'Морозова', 'Ивановна'),
('c6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Светлана', 'Волкова', 'Павловна'),
('c7eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Наталья', 'Зайцева', 'Михайловна'),
('c8eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Екатерина', 'Соколова', 'Дмитриевна'),
('c9eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Александр', 'Лебедев', 'Игоревич'),
('ca0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Дмитрий', 'Кузнецов', 'Олегович');

-- 5. Создаем сотрудников
INSERT INTO employees (id, created_at, updated_at, contact_phone, contact_email, contact_telegram, is_owner, is_manager, is_master, organization_id, manager_branch_id, user_id) 
VALUES
('d0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234567', 'owner@roga-kopyta.ru', NULL, true, false, false, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234568', 'anna@roga-kopyta.ru', '@anna_master', false, false, true, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234569', 'maria@roga-kopyta.ru', '@maria_master', false, false, true, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234570', 'elena@roga-kopyta.ru', NULL, false, false, true, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234571', 'olga@roga-kopyta.ru', '@olga_master', false, false, true, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('d5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234572', 'tatiana@roga-kopyta.ru', NULL, false, false, true, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), '+79271234573', 'svetlana@roga-kopyta.ru', '@svetlana_master', false, false, true, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NULL, 'c6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11');

-- 6. Создаем услуги
INSERT INTO services (id, created_at, updated_at, display_name, description, duration_minutes, price, master_id) 
VALUES
('e1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Стрижка женская', 'Стрижка с консультацией стилиста', 60, 2500.00, 'd1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Окрашивание волос', 'Окрашивание в один тон', 180, 5000.00, 'd1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Укладка', 'Праздничная укладка', 45, 1800.00, 'd1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Стрижка женская', 'Модельная стрижка', 60, 2300.00, 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Стрижка мужская', 'Мужская стрижка с укладкой', 30, 1500.00, 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Маникюр', 'Маникюр с покрытием гель-лак', 90, 2000.00, 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e7eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Педикюр', 'Педикюр с покрытием', 120, 2500.00, 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e8eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Окрашивание волос', 'Сложное окрашивание', 240, 7000.00, 'd4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('e9eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Мелирование', 'Калифорнийское мелирование', 180, 6000.00, 'd4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('ea0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Стрижка женская', 'Стрижка горячими ножницами', 75, 2800.00, 'd5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('eb0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Стрижка детская', 'Детская стрижка', 30, 1200.00, 'd5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('ec0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Маникюр', 'Европейский маникюр', 60, 1500.00, 'd6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('ed0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'Наращивание ногтей', 'Наращивание гелем', 150, 3500.00, 'd6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11');

-- 7. Создаем расписания
INSERT INTO timetables (master_id, recurrence_cycle_start) 
VALUES 
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27'),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27'),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27'),
('d5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27'),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27');

-- 8. Создаем расписание для Анны (пн-пт 9:00-19:00, сб 10:00-16:00, вс выходной)
INSERT INTO schedule_days (master_id, created_at, updated_at, day_ordinal, day_data) 
VALUES 
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 0, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-27T09:00:00", "end": "2025-01-27T19:00:00"}, "breakIntervals": [{"start": "2025-01-27T13:00:00", "end": "2025-01-27T14:00:00"}]}'::jsonb),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 1, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-28T09:00:00", "end": "2025-01-28T19:00:00"}, "breakIntervals": [{"start": "2025-01-28T13:00:00", "end": "2025-01-28T14:00:00"}]}'::jsonb),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 2, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-29T09:00:00", "end": "2025-01-29T19:00:00"}, "breakIntervals": [{"start": "2025-01-29T13:00:00", "end": "2025-01-29T14:00:00"}]}'::jsonb),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 3, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-30T09:00:00", "end": "2025-01-30T19:00:00"}, "breakIntervals": [{"start": "2025-01-30T13:00:00", "end": "2025-01-30T14:00:00"}]}'::jsonb),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 4, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-31T09:00:00", "end": "2025-01-31T19:00:00"}, "breakIntervals": [{"start": "2025-01-31T13:00:00", "end": "2025-01-31T14:00:00"}]}'::jsonb),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 5, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-02-01T10:00:00", "end": "2025-02-01T16:00:00"}, "breakIntervals": []}'::jsonb),
('d1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 6, '{"dayType": "weekend"}'::jsonb);

-- 9. Создаем расписание для Марии (вт-сб 10:00-20:00, пн,вс выходной)
INSERT INTO schedule_days (master_id, created_at, updated_at, day_ordinal, day_data) 
VALUES 
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 0, '{"dayType": "weekend"}'::jsonb),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 1, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-28T10:00:00", "end": "2025-01-28T20:00:00"}, "breakIntervals": [{"start": "2025-01-28T14:00:00", "end": "2025-01-28T15:00:00"}]}'::jsonb),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 2, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-29T10:00:00", "end": "2025-01-29T20:00:00"}, "breakIntervals": [{"start": "2025-01-29T14:00:00", "end": "2025-01-29T15:00:00"}]}'::jsonb),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 3, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-30T10:00:00", "end": "2025-01-30T20:00:00"}, "breakIntervals": [{"start": "2025-01-30T14:00:00", "end": "2025-01-30T15:00:00"}]}'::jsonb),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 4, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-31T10:00:00", "end": "2025-01-31T20:00:00"}, "breakIntervals": [{"start": "2025-01-31T14:00:00", "end": "2025-01-31T15:00:00"}]}'::jsonb),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 5, '{"dayType": "weekday", "branchId": "b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-02-01T10:00:00", "end": "2025-02-01T18:00:00"}, "breakIntervals": []}'::jsonb),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 6, '{"dayType": "weekend"}'::jsonb);

-- 10. Создаем расписание для Елены (пн-пт 11:00-19:00, сб-вс выходной)
INSERT INTO schedule_days (master_id, created_at, updated_at, day_ordinal, day_data) 
SELECT 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), gs-1, 
  CASE 
    WHEN gs IN (1,2,3,4,5) THEN jsonb_build_object(
      'dayType', 'weekday', 
      'branchId', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 
      'workingInterval', jsonb_build_object(
        'start', '2025-01-27T11:00:00'::timestamp + (gs-1) * interval '1 day', 
        'end', '2025-01-27T19:00:00'::timestamp + (gs-1) * interval '1 day'
      ), 
      'breakIntervals', jsonb_build_array(
        jsonb_build_object(
          'start', '2025-01-27T15:00:00'::timestamp + (gs-1) * interval '1 day', 
          'end', '2025-01-27T15:30:00'::timestamp + (gs-1) * interval '1 day'
        )
      )
    ) 
    ELSE jsonb_build_object('dayType', 'weekend') 
  END 
FROM generate_series(1,7) gs;

-- 11. Создаем расписание для Ольги (пн,ср,пт 9:00-18:00)
INSERT INTO schedule_days (master_id, created_at, updated_at, day_ordinal, day_data) 
VALUES 
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 0, '{"dayType": "weekday", "branchId": "b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-27T09:00:00", "end": "2025-01-27T18:00:00"}, "breakIntervals": [{"start": "2025-01-27T13:00:00", "end": "2025-01-27T14:00:00"}]}'::jsonb),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 1, '{"dayType": "weekend"}'::jsonb),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 2, '{"dayType": "weekday", "branchId": "b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-29T09:00:00", "end": "2025-01-29T18:00:00"}, "breakIntervals": [{"start": "2025-01-29T13:00:00", "end": "2025-01-29T14:00:00"}]}'::jsonb),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 3, '{"dayType": "weekend"}'::jsonb),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 4, '{"dayType": "weekday", "branchId": "b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-31T09:00:00", "end": "2025-01-31T18:00:00"}, "breakIntervals": [{"start": "2025-01-31T13:00:00", "end": "2025-01-31T14:00:00"}]}'::jsonb),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 5, '{"dayType": "weekend"}'::jsonb),
('d4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 6, '{"dayType": "weekend"}'::jsonb);

-- 12. Создаем расписание для Татьяны (пн-пт 8:00-16:00)
INSERT INTO schedule_days (master_id, created_at, updated_at, day_ordinal, day_data) 
SELECT 'd5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), gs-1, 
  CASE 
    WHEN gs <= 5 THEN jsonb_build_object(
      'dayType', 'weekday', 
      'branchId', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 
      'workingInterval', jsonb_build_object(
        'start', '2025-01-27T08:00:00'::timestamp + (gs-1) * interval '1 day', 
        'end', '2025-01-27T16:00:00'::timestamp + (gs-1) * interval '1 day'
      ), 
      'breakIntervals', jsonb_build_array(
        jsonb_build_object(
          'start', '2025-01-27T12:00:00'::timestamp + (gs-1) * interval '1 day', 
          'end', '2025-01-27T12:30:00'::timestamp + (gs-1) * interval '1 day'
        )
      )
    ) 
    ELSE jsonb_build_object('dayType', 'weekend') 
  END 
FROM generate_series(1,7) gs;

-- 13. Создаем расписание для Светланы (вт,чт,сб 12:00-20:00)
INSERT INTO schedule_days (master_id, created_at, updated_at, day_ordinal, day_data) 
VALUES 
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 0, '{"dayType": "weekend"}'::jsonb),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 1, '{"dayType": "weekday", "branchId": "b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-28T12:00:00", "end": "2025-01-28T20:00:00"}, "breakIntervals": [{"start": "2025-01-28T16:00:00", "end": "2025-01-28T16:30:00"}]}'::jsonb),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 2, '{"dayType": "weekend"}'::jsonb),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 3, '{"dayType": "weekday", "branchId": "b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-01-30T12:00:00", "end": "2025-01-30T20:00:00"}, "breakIntervals": [{"start": "2025-01-30T16:00:00", "end": "2025-01-30T16:30:00"}]}'::jsonb),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 4, '{"dayType": "weekend"}'::jsonb),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 5, '{"dayType": "weekday", "branchId": "b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", "workingInterval": {"start": "2025-02-01T12:00:00", "end": "2025-02-01T20:00:00"}, "breakIntervals": []}'::jsonb),
('d6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 6, '{"dayType": "weekend"}'::jsonb);

-- 14. Создаем клиентов
INSERT INTO customers (id, created_at, updated_at, organization_id, user_id) 
VALUES 
('f1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'c7eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'c8eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('f3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'c9eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('f4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'ca0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11');

-- 15. Создаем бронирования для имитации занятости
INSERT INTO bookings (id, created_at, updated_at, customer_id, service_id, master_id, branch_id, started_at, ended_at, notify_methods) 
VALUES 
-- Бронирования на понедельник 27 января
('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27 10:00:00', '2025-01-27 11:00:00', '{sms}'),
('a2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27 14:00:00', '2025-01-27 17:00:00', '{telegram}'),
('a3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27 11:30:00', '2025-01-27 13:00:00', '{sms,telegram}'),
('a4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'eb0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-27 09:00:00', '2025-01-27 09:30:00', '{sms}'),
-- Бронирования на вторник 28 января
('a5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e5eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-28 10:30:00', '2025-01-28 11:00:00', '{sms}'),
('a6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-28 16:00:00', '2025-01-28 17:00:00', '{telegram}'),
('a7eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'ec0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd6eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-28 14:00:00', '2025-01-28 15:00:00', '{sms}'),
-- Бронирования на среду 29 января
('a8eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e9eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd4eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-29 09:00:00', '2025-01-29 12:00:00', '{telegram}'),
('a9eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd1eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-29 11:00:00', '2025-01-29 11:45:00', '{sms}'),
-- Бронирование на пятницу 31 января
('aa0ebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW(), NOW(), 'f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'e7eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2025-01-31 15:30:00', '2025-01-31 17:30:00', '{sms,telegram}');

-- Проверка данных
SELECT 'Данные успешно загружены!' as message;
SELECT 'Организаций:' as entity, count(*) as count FROM organizations
UNION ALL
SELECT 'Филиалов:', count(*) FROM branches
UNION ALL
SELECT 'Пользователей:', count(*) FROM users
UNION ALL
SELECT 'Профилей:', count(*) FROM user_profiles
UNION ALL
SELECT 'Сотрудников:', count(*) FROM employees
UNION ALL
SELECT 'Услуг:', count(*) FROM services
UNION ALL
SELECT 'Расписаний:', count(*) FROM timetables
UNION ALL
SELECT 'Дней расписания:', count(*) FROM schedule_days
UNION ALL
SELECT 'Клиентов:', count(*) FROM customers
UNION ALL
SELECT 'Бронирований:', count(*) FROM bookings;