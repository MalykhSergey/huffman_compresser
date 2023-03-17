# Курсовой проект за 3 семестр. Сжатие данных методом Хаффмана на ЯП Rust. 
## Результаты тестирования
Для тестирования были взяты несколько файлов. Первый из них – это текстовый файл содержащий роман Л.Н. Толстого "Война и мир". До сжатия его размер составлял
3047 КБ. После сжатия размер файла составил 1895 КБ. Время затраченное на сжатие составило 85 миллисекунд. Время затраченное на разжатие составило 75
миллисекунд. Скорее всего время сжатия оказалось больше из-за того, что сжатый файл прочитался и записался в разжатый быстрее из-за своего гораздо меньшего
размера. Ведь коэффицент сжатия составил 1,6 раз (отношения разжатого файла к сжатому файлу).

Второй файл это это уже сжатый zip архив размером 3 091 844 КБ. Размер сжатого файла составил 3 091 846 КБ. Время сжатия равно 55 секундам. Время разжатия равно
99 секундам. Хотя сильнее сжать архив уже не получилось, его размер вырос всего на 2 КБ (размер заголовка при использовании 256 символов). 

Третий файл это почти код в азбуке Морзе, содержащий символы тире, точки и пробелы. Размер исходного файла составил 4709 КБ. С этим файлом производились
следующие действия:

1. Файл сжали. Размер сжатого файла составил 793 КБ. То есть коэффицент сжатия составил почти 5,94 раз. Время сжатия заняло 78 29 миллисекунд.
2. Затем сжатый файл был переименован и сжат ещё раз. Размер нового
сжатого файла составил 523 КБ. Время сжатия 29 миллисекунд.
3. Затем сжатый файл снова переименовали и сжали ещё раз. Размер
нового сжатого файла составил 492 КБ. Время сжатия 26 миллисекунд.
4. Затем сжатый файл снова переименовали и сжали ещё раз. Размер
нового сжатого файла составил 496 КБ. Время сжатия 22 миллисекунды.
5. Последний файл разжали за 33 миллисекунды.
6. Разжатый файл переименовали и снова разжали за 33 миллисекунды.
7. Разжатый файл переименовали и снова разжали за 29 миллисекунд.
8. Разжатый файл переименовали и снова разжали за 23 миллисекунды.
9. Разжатый файл переименовали и снова разжали за 28 миллисекунд.

После выполнения этой последовательности действий получили файл идентичный исходному, но с другим именем.

После программа была испытана с поврежденным файлом (сжатым романом). Для его повреждения необходимо открыть hfm файл в Hex-редакторе и изменить несколько
байтов. Несмотря на несколько попыток повреждения файла в результате разжатия всегда получалось частично восстановить исходный файл. В зависимости от
поврежденных данных изменялся либо размер файла (если поменяли первые 8 байт), либо незначительно его содержание (если были затронут сами сжатые данные, но не
массив символов с частотами), либо серьёзно отличалось содержание (если серьёзно повредить массив символов с частотами)

## Заключение
... Реализация программы, производящей сжатие и разжатие данных по методу Хаффмана, на языке программирования Rust оказалась одновременно достаточно
эффективной, простой и небольшой. Количество всех строк кода с подключением всех модулей и форматированием программы составило всего 311 штук. А размер
скомпилированной программы с флагом --release составил всего 208 КБ. Этот размер при желании можно уменьшить ещё больше, но особой необходимости в этом нет...

И хотя получившаяся программа не имеет графического интерфейса, для её запуска не обязательно пользоваться командной строкой. Для её работы нужно лишь выделять
файлы предназначенные для сжатия или разжатия (можно вперемешку) и перетянуть их на программу по принципу Drag&Drop. После работы программы в директории файла
появится новый файл (сжатый или разжатый зависит от ситуации).
