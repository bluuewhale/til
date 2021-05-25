package com.donghyungko.jpabook.jpashop.repository;

import com.donghyungko.jpabook.jpashop.domain.item.Item;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import org.springframework.stereotype.Repository;

import javax.persistence.EntityManager;
import java.util.List;
import java.util.Optional;

@Repository
@RequiredArgsConstructor
public class ItemRepository {
    private final EntityManager em;


    public void save(Item item) {
        if (item.getId() == null) {
            em.persist(item); // new entry
        } else {
            em.merge(item); // update
        }
    }

    public Optional<Item> findOne(Long id) {
        return Optional.ofNullable(em.find(Item.class, id));
    }

    public List<Item> findAll() {
        return em.createQuery("select i from Item i").getResultList();
    }

    public List<Item> findByName(String name) {
        return em.createQuery("select i from Item i where i.name = :name")
                .setParameter("name", name)
                .getResultList();

    }
}
