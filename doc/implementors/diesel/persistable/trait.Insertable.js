(function() {var implementors = {};
implementors["rusty_nail"] = ["impl&lt;'a, 'insert, DB&gt; Insertable&lt;<a class='struct' href='rusty_nail/recipe/schema/recipes/struct.table.html' title='rusty_nail::recipe::schema::recipes::table'>table</a>, DB&gt; for &amp;'insert <a class='struct' href='rusty_nail/recipe/models/struct.NewRecipe.html' title='rusty_nail::recipe::models::NewRecipe'>NewRecipe</a>&lt;'a&gt; <span class='where fmt-newline'>where DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>(</a>ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/recipes/struct.name.html' title='rusty_nail::recipe::schema::recipes::name'>name</a>, AsExpr&lt;&amp;'insert &amp;'a <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>, <a class='struct' href='rusty_nail/recipe/schema/recipes/struct.name.html' title='rusty_nail::recipe::schema::recipes::name'>name</a>&gt;&gt;, ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/recipes/struct.description.html' title='rusty_nail::recipe::schema::recipes::description'>description</a>, AsExpr&lt;&amp;'insert &amp;'a <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>, <a class='struct' href='rusty_nail/recipe/schema/recipes/struct.description.html' title='rusty_nail::recipe::schema::recipes::description'>description</a>&gt;&gt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>)</a>: InsertValues&lt;DB&gt;</span>","impl&lt;'insert, DB&gt; Insertable&lt;<a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.table.html' title='rusty_nail::recipe::schema::recipe_ingredients::table'>table</a>, DB&gt; for &amp;'insert <a class='struct' href='rusty_nail/recipe/models/struct.NewRecipeIngredient.html' title='rusty_nail::recipe::models::NewRecipeIngredient'>NewRecipeIngredient</a> <span class='where fmt-newline'>where DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>(</a>ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.recipe_id.html' title='rusty_nail::recipe::schema::recipe_ingredients::recipe_id'>recipe_id</a>, AsExpr&lt;&amp;'insert <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.i32.html'>i32</a>, <a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.recipe_id.html' title='rusty_nail::recipe::schema::recipe_ingredients::recipe_id'>recipe_id</a>&gt;&gt;, ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.ingredient_id.html' title='rusty_nail::recipe::schema::recipe_ingredients::ingredient_id'>ingredient_id</a>, AsExpr&lt;&amp;'insert <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.i32.html'>i32</a>, <a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.ingredient_id.html' title='rusty_nail::recipe::schema::recipe_ingredients::ingredient_id'>ingredient_id</a>&gt;&gt;, ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.amount.html' title='rusty_nail::recipe::schema::recipe_ingredients::amount'>amount</a>, AsExpr&lt;&amp;'insert <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.f32.html'>f32</a>, <a class='struct' href='rusty_nail/recipe/schema/recipe_ingredients/struct.amount.html' title='rusty_nail::recipe::schema::recipe_ingredients::amount'>amount</a>&gt;&gt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>)</a>: InsertValues&lt;DB&gt;</span>","impl&lt;'a, 'insert, DB&gt; Insertable&lt;<a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.table.html' title='rusty_nail::recipe::schema::ingredients::table'>table</a>, DB&gt; for &amp;'insert <a class='struct' href='rusty_nail/recipe/models/struct.NewIngredient.html' title='rusty_nail::recipe::models::NewIngredient'>NewIngredient</a>&lt;'a&gt; <span class='where fmt-newline'>where DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>(</a>ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.name.html' title='rusty_nail::recipe::schema::ingredients::name'>name</a>, AsExpr&lt;&amp;'insert &amp;'a <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>, <a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.name.html' title='rusty_nail::recipe::schema::ingredients::name'>name</a>&gt;&gt;, ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.description.html' title='rusty_nail::recipe::schema::ingredients::description'>description</a>, AsExpr&lt;&amp;'insert &amp;'a <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>, <a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.description.html' title='rusty_nail::recipe::schema::ingredients::description'>description</a>&gt;&gt;, ColumnInsertValue&lt;<a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.available.html' title='rusty_nail::recipe::schema::ingredients::available'>available</a>, AsExpr&lt;&amp;'insert <a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.bool.html'>bool</a>, <a class='struct' href='rusty_nail/recipe/schema/ingredients/struct.available.html' title='rusty_nail::recipe::schema::ingredients::available'>available</a>&gt;&gt;<a class='primitive' href='https://doc.rust-lang.org/nightly/std/primitive.tuple.html'>)</a>: InsertValues&lt;DB&gt;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()