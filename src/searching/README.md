## Search Algorithms

* [Linear-search](#linear)
* [Binary-search](#binary)
* [Exponential-search](#exponential)

* * *

[<h3 id="linear">Linear</h3>](./linear_search.rs)

![linear_search][linear_image]

**Линейный, последовательный поиск** — алгоритм нахождения заданного значения произвольной функции на некотором отрезке. Данный алгоритм является простейшим алгоритмом поиска и, в отличие, например, от [бинарного поиска](#binary), не накладывает никаких ограничений на функцию и имеет простейшую реализацию. 

Поиск значения функции осуществляется простым сравнением очередного рассматриваемого значения (как правило, поиск происходит слева направо, то есть от меньших значений аргумента к большим) и, если значения совпадают (с той или иной точностью), то поиск считается завершённым.

__Time complexity__
* Худшая скорость O(n)
* Лучшая скорость O(1)
* Средняя скорость O(n)

__Memory__
* O(1)

* * *

[<h3 id="binary">Binary</h3>](./binary_search.rs)

![binary_search][binary_image]

**Двоичный (бинарный) поиск** — классический алгоритм поиска элемента в отсортированном массиве, использующий дробление массива на половины.

Алгоритм заключается в следующем:
* Определение значения элемента в середине структуры данных. Полученное значение сравнивается с ключом.
* Если ключ меньше значения середины, то поиск осуществляется в первой половине элементов, иначе — во второй.
* Поиск сводится к тому, что вновь определяется значение серединного элемента в выбранной половине и сравнивается с ключом.
* Процесс продолжается до тех пор, пока не будет найден элемент со значением ключа или не станет пустым интервал для поиска.

__Time complexity__
* Худшая скорость O(log n)
* Лучшая скорость O(1)
* Средняя скорость O(log n)

__Memory__
* O(1)

* * *

[<h3 id="exponential">Exponential</h3>](./exponential_search.rs)

![exponential_search][exponential_image]

**Экспоенциальный поиск** - это поиск, расчитанный на бесконечные массивы.

Состоит алгоритм в следующем:
* используя степень 2, ищутся индексы элементов, между которых _может_ располагаться искомый элемент
* к данному промежутку применяется обычный бинарный поиск

__Time complexity__
* Худшая скорость O(log i)
* Лучшая скорость O(1)
* Средняя скорость O(log i)

где `i` - индекс искомого элемента в массиве, или же индекс позиции, где элемент мог бы располагаться.

__Memory__
* O(1)

<!--
IMAGE_ID
-->

[linear_image]: https://camo.githubusercontent.com/5cfe6f9610708af79ad630ab47faf788eb600b6dfe543903492675780aecc11d/68747470733a2f2f7777772e7475746f7269616c73706f696e742e636f6d2f646174615f737472756374757265735f616c676f726974686d732f696d616765732f6c696e6561725f7365617263682e676966 "Linear Search"
[binary_image]: https://upload.wikimedia.org/wikipedia/commons/8/83/Binary_Search_Depiction.svg "Binary Search"
[exponential_image]: https://upload.wikimedia.org/wikipedia/commons/4/45/Exponential_search.svg "Exponential Search"
