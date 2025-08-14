# Backend

## Key dependencies

1. Programming Language: Rust
2. Asynchronous Runtime: tokio
3. Framework: Poem OpenAPI
4. Database: PostgreSQL
5. Database Queries: sqlx
6. Time Handling: chrono
7. Data Validation: garde
8. Identifier Work: uuid
9. Builder Creation: bon
10. Parameterized Tests: yare

## Testing strategy

## Testing Strategy
The codebase uses `yare` for parameterized tests extensively. Tests are located in the same files as the implementation using `#[cfg(test)]` modules.

### Best Practices for Writing Tests

1. **Use Parameterized Tests Wisely**
   - Use `yare::parameterized` for testing multiple similar scenarios
   - Avoid if/else or match statements based on test parameters - this is an anti-pattern
   - If you need conditional logic, split into separate test functions
   - For async parameterized tests: Use `#[parameterized(...)]` followed by `#[test_macro(tokio::test)]` (not just `#[tokio::test]`)

2. **Builder Pattern for Test Data**
   - Create test-specific builder functions to construct test data
   - Use default parameters to reduce boilerplate
   - Example:
   ```rust
   #[builder(finish_fn = build)]
   fn test_schedule_date(
       #[builder(default = 5)] weekdays: usize,
       #[builder(default = 2)] weekends: usize,
       #[builder(default = NaiveDate::from_ymd_opt(2025, 8, 11).unwrap())] cycle_start: NaiveDate,
   ) -> TestScheduleData { ... }
   ```

3. **Separate Tests by Concern**
   - Split tests that check different behaviors into separate functions
   - Example: Instead of one test with if/else for weekdays vs weekends, create:
     - `test_get_schedule_for_date_weekdays`
     - `test_get_schedule_for_date_weekends`
     - `test_get_schedule_for_date_with_breaks`

4. **Avoid Overly Complex Parameterization**
   - If parameterized test requires matching on parameters to verify results, consider splitting it
   - Each parameterized test case should follow the same assertion pattern

5. **Naming Conventions**
   - Parameterized test cases should have descriptive names
   - Test function names should clearly indicate what is being tested
   - Use consistent prefixes: `test_`, `create_`, etc.
