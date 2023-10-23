Попытка разработать подход к программированию на Rust в стиле промышленных ПЛК.

## Структурные единицы

- Организационные блоки (OB)
- Функциональные блоки (FB)

### Организационные блоки (OB)

Верхнеуровневые элементы. Выполнены в виде задач tokio. Параллельно можно запускать несколько OB. Поскольку задачи выполняются в разных потоках, нет необходимости вытеснять OB по приоритетам - они выполняются параллельно.

plc-rs не заботится, откуда данные приходят и куда отправляются. Входные данные поступают из брокера сообщений и передаются по каналу в OB. Выходные данные также отправляются в брокер сообщений. Коммуникации с устройствами также программируются во внешних крейтах.

### Функциональные блоки (FB)

Базовые компоненты для построения программы. Сохраняют состояние между вызовами.
