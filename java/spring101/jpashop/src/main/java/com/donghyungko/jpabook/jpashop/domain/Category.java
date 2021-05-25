package com.donghyungko.jpabook.jpashop.domain;

import com.donghyungko.jpabook.jpashop.domain.item.Item;
import com.sun.istack.NotNull;
import lombok.*;
import org.springframework.beans.factory.annotation.Autowired;

import javax.persistence.*;
import java.util.ArrayList;
import java.util.List;

@Entity
@Getter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class Category {

    @Builder
    public Category(Long id, String name, List<Item> items, Category parent, List<Category> child) {
        this.id = id;
        this.name = name;
        this.items = items;
        this.parent = parent;
        this.child = child;
    }

    @Id @GeneratedValue
    @Column(name = "category_id")
    private Long id;

    @Column(name = "name")
    private String name;

    @ManyToMany // 사용하지 않는 것이 좋다.
    @JoinTable(name = "category_item", // JoinTable 방식의 다대다 매칭
            joinColumns = @JoinColumn(name = "category_id"),
            inverseJoinColumns = @JoinColumn(name = "item_id")
    )
    private List<Item> items = new ArrayList<>();


    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "parent_id")
    private Category parent;

    @OneToMany(mappedBy = "parent")
    private List<Category> child = new ArrayList<>();

    // 연관관계 메서드
    public void addChildCategory(Category child) {
        this.child.add(child);
        child.updateParent(this);
    }

    public void updateParent(Category parent) {
        this.parent = parent;
    }

    public void addItem(Item item) {
        items.add(item);
    }

    public void addChild(Category category) {
        this.child.add(category);
    }
}
