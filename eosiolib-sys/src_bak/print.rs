// https://developers.eos.io/eosio-cpp/reference#console

extern "C" {
    /**
     *  @defgroup consoleapi Console API
     *  @brief Defines APIs to log/print text messages
     *  @ingroup contractdev
     *
     */

    /**
     *  @defgroup consolecapi Console C API
     *  @brief Defnes %C API to log/print text messages
     *  @ingroup consoleapi
     *  @{
     */

    /**
     *  Prints string
     *  @brief Prints string
     *  @param cstr - a null terminated string
     *
     *  Example:
     *
     *  @code
     *  prints("Hello World!"); // Output: Hello World!
     *  @endcode
     */
    pub fn prints(message: *const u8);

    /**
     *  Prints string up to given length
     *  @brief Prints string
     *  @param cstr - pointer to string
     *  @param len - len of string to be printed
     *
     *  Example:
     *
     *  @code
     *  prints_l("Hello World!", 5); // Output: Hello
     *  @endcode
     */
    pub fn prints_l(cstr: *const u8, len: usize);

    /**
     * Prints value as a 64 bit signed integer
     * @brief Prints value as a 64 bit signed integer
     * @param value of 64 bit signed integer to be printed
     *
     *  Example:
     *
     *  @code
     *  printi(-1e+18); // Output: -1000000000000000000
     *  @endcode
     */
    pub fn printi(value: i64);

    /**
     * Prints value as a 64 bit unsigned integer
     * @brief Prints value as a 64 bit unsigned integer
     * @param value of 64 bit unsigned integer to be printed
     *
     *  Example:
     *
     *  @code
     *  printui(1e+18); // Output: 1000000000000000000
     *  @endcode
     */
    pub fn printui(value: u64);

    /**
     * Prints value as a 128 bit signed integer
     * @brief Prints value as a 128 bit signed integer
     * @param value is a pointer to the 128 bit signed integer to be printed
     *
     *  Example:
     *
     *  @code
     *  int128 large_int(-87654323456);
     *  printi128(&large_int); // Output: -87654323456
     *  @endcode
     */
    // pub fn printi128(value: *const i128);

    /**
     * Prints value as a 128 bit unsigned integer
     * @brief Prints value as a 128 bit unsigned integer
     * @param value is a pointer to the 128 bit unsigned integer to be printed
     *
     *  Example:
     *
     *  @code
     *  u128 large_int(87654323456);
     *  printui128(&large_int); // Output: 87654323456
     *  @endcode
     */
    //    pub fn printui128(value: *const u128);

    /**
     * Prints value as single-precision floating point number
     * @brief Prints value as single-precision floating point number (i.e. float)
     * @param value of float to be printed
     *
     *  Example:
     *
     *  @code
     *  float value = 5.0 / 10.0;
     *  printsf(value); // Output: 0.5
     *  @endcode
     */
    pub fn printsf(value: f32);

    /**
     * Prints value as double-precision floating point number
     * @brief Prints value as double-precision floating point number (i.e. double)
     * @param value of double to be printed
     *
     *  Example:
     *
     *  @code
     *  double value = 5.0 / 10.0;
     *  printdf(value); // Output: 0.5
     *  @endcode
     */
    pub fn printdf(value: f64);

    /**
     * Prints value as quadruple-precision floating point number
     * @brief Prints value as quadruple-precision floating point number (i.e. long double)
     * @param value is a pointer to the long double to be printed
     *
     *  Example:
     *
     *  @code
     *  long double value = 5.0 / 10.0;
     *  printqf(value); // Output: 0.5
     *  @endcode
     */
    //    fn printqf(const long double* value);

    /**
     * Prints a 64 bit names as base32 encoded string
     * @brief Prints a 64 bit names as base32 encoded string
     * @param name - 64 bit name to be printed
     *
     * Example:
     * @code
     * printn(N(abcde)); // Output: abcde
     * @endcode
     */
    pub fn printn(name: u64);

//fn printhex( const void* data, u32 datalen );

}
