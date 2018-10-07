$(function () {
    var $input = $('.form__input, .form__textarea'),
        $group = $('.form__group');
    $($input).on("focus", function () {
        $(this).parents($group).addClass('is-focus');
    });
    $($input).on("blur", function () {
        $(this).parents($group).removeClass('is-focus');
        if ($(this).val()) {
            $(this).parents($group).addClass('is-value');
        } else {
            $(this).parents($group).removeClass('is-value');
        }

    });
});

